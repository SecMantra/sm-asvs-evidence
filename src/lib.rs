//! Embeddable, deterministic ASVS-style evidence assessment library.
//! A PASS only means the configured policy was satisfied by supplied evidence.
//! Inputs are not authenticated and output is not an ASVS certification.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Catalogue { pub version: String, pub requirements: Vec<Requirement> }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Requirement { pub id: String, pub text: String, pub level: u8 }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceFile { pub application: String, pub commit: String, pub evidence: Vec<Evidence> }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence { pub id: String, pub requirement_id: String, pub kind: String, pub outcome: String, pub commit: String, pub source: String }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyFile { pub rules: Vec<Rule> }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rule { pub requirement_id: String, pub required_kinds: Vec<String>, #[serde(default)] pub human_review: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment { pub requirement_id: String, pub status: String, pub rationale: String, pub evidence_ids: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report { pub schema_version: String, pub application: String, pub commit: String, pub catalogue_version: String, pub assessments: Vec<Assessment> }

pub fn validate(c: &Catalogue, e: &EvidenceFile, p: &PolicyFile) -> Result<(), String> {
    if c.version.trim().is_empty() || c.requirements.is_empty() { return Err("empty catalogue/version".into()); }
    if e.application.trim().is_empty() || e.commit.trim().is_empty() { return Err("application and commit required".into()); }
    let mut ids = HashSet::new();
    for r in &c.requirements {
        if r.id.trim().is_empty() || r.text.trim().is_empty() || !(1..=3).contains(&r.level) { return Err(format!("invalid requirement {}", r.id)); }
        if !ids.insert(r.id.as_str()) { return Err(format!("duplicate requirement {}", r.id)); }
    }
    let mut rule_ids = HashSet::new();
    for rule in &p.rules {
        if !ids.contains(rule.requirement_id.as_str()) || !rule_ids.insert(rule.requirement_id.as_str()) { return Err(format!("unknown or duplicate policy {}", rule.requirement_id)); }
        if rule.required_kinds.is_empty() || rule.required_kinds.iter().any(|k| k.trim().is_empty()) { return Err(format!("empty required evidence kinds for {}", rule.requirement_id)); }
    }
    let mut evidence_ids = HashSet::new();
    for item in &e.evidence {
        if !evidence_ids.insert(item.id.as_str()) || item.id.trim().is_empty() { return Err(format!("duplicate or empty evidence id {}", item.id)); }
        if !ids.contains(item.requirement_id.as_str()) { return Err(format!("unknown evidence requirement {}", item.requirement_id)); }
        if !matches!(item.outcome.as_str(), "passed" | "failed" | "unknown") { return Err(format!("invalid outcome for {}", item.id)); }
        if item.kind.trim().is_empty() || item.commit.trim().is_empty() || item.source.trim().is_empty() { return Err(format!("incomplete evidence {}", item.id)); }
    }
    Ok(())
}

/// Evaluates a user-supplied policy and evidence set; does not authenticate evidence provenance.
pub fn assess(c: &Catalogue, e: &EvidenceFile, p: &PolicyFile) -> Result<Report, String> {
    validate(c, e, p)?;
    let rules: BTreeMap<_, _> = p.rules.iter().map(|r| (r.requirement_id.as_str(), r)).collect();
    let assessments = c.requirements.iter().map(|r| {
        let relevant: Vec<_> = e.evidence.iter().filter(|x| x.requirement_id == r.id && x.commit == e.commit).collect();
        let evidence_ids = relevant.iter().map(|x| x.id.clone()).collect();
        let (status, rationale) = match rules.get(r.id.as_str()) {
            None => ("NO EVIDENCE", "No assessment policy defined"),
            Some(_) if relevant.is_empty() => ("NO EVIDENCE", "No evidence for assessed commit"),
            Some(_) if relevant.iter().any(|x| x.outcome == "failed") => ("FAIL", "Current evidence reports failure"),
            Some(rule) if rule.human_review => ("NEEDS REVIEW", "Human verification required"),
            Some(rule) if rule.required_kinds.iter().all(|k| relevant.iter().any(|x| &x.kind == k && x.outcome == "passed")) => ("PASS", "Configured evidence policy satisfied; provenance not verified"),
            Some(_) => ("NEEDS REVIEW", "Insufficient or ambiguous evidence"),
        };
        Assessment { requirement_id: r.id.clone(), status: status.into(), rationale: rationale.into(), evidence_ids }
    }).collect();
    Ok(Report { schema_version: "1.0".into(), application: e.application.clone(), commit: e.commit.clone(), catalogue_version: c.version.clone(), assessments })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Catalogue, EvidenceFile, PolicyFile) {
        (serde_json::from_str(include_str!("../fixtures/catalogue.json")).unwrap(),
         serde_json::from_str(include_str!("../fixtures/evidence.json")).unwrap(),
         serde_json::from_str(include_str!("../fixtures/policy.json")).unwrap())
    }
    #[test] fn expected_outcomes() {
        let (c,e,p)=fixture(); let r=assess(&c,&e,&p).unwrap();
        assert_eq!(r.assessments.iter().map(|x| x.status.as_str()).collect::<Vec<_>>(), vec!["PASS","FAIL","NEEDS REVIEW","NO EVIDENCE"]);
    }
    #[test] fn stale_evidence_never_passes() {
        let (c,mut e,p)=fixture(); e.commit="new-commit".into();
        assert!(assess(&c,&e,&p).unwrap().assessments.iter().all(|x| x.status=="NO EVIDENCE"));
    }
    #[test] fn failed_evidence_overrides_pass() {
        let (c,mut e,p)=fixture(); e.evidence[0].outcome="failed".into();
        assert_eq!(assess(&c,&e,&p).unwrap().assessments[0].status,"FAIL");
    }
    #[test] fn invalid_evidence_is_rejected() {
        let (c,mut e,p)=fixture(); e.evidence[0].outcome="green".into();
        assert!(assess(&c,&e,&p).is_err());
    }
    #[test] fn empty_required_kinds_cannot_pass() {
        let (c,e,mut p)=fixture(); p.rules[0].required_kinds.clear();
        assert!(assess(&c,&e,&p).is_err());
    }
    #[test] fn duplicate_ids_rejected() {
        let (c,mut e,p)=fixture(); e.evidence.push(e.evidence[0].clone());
        assert!(assess(&c,&e,&p).is_err());
    }
}
