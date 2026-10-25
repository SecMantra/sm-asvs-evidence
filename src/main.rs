use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{collections::{BTreeMap, HashSet}, fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "asvs-evidence", version, about = "Evidence-backed ASVS assessments (PoC)")]
struct Cli { #[command(subcommand)] command: Command }
#[derive(Subcommand)]
enum Command {
    Catalogue { #[arg(long)] input: PathBuf },
    Assess { #[arg(long)] catalogue: PathBuf, #[arg(long)] evidence: PathBuf, #[arg(long)] policy: PathBuf, #[arg(long)] output: Option<PathBuf> },
}
#[derive(Debug, Deserialize)]
struct Catalogue { version: String, requirements: Vec<Requirement> }
#[derive(Debug, Deserialize)]
struct Requirement { id: String, text: String, level: u8 }
#[derive(Debug, Deserialize)]
struct EvidenceFile { application: String, commit: String, evidence: Vec<Evidence> }
#[derive(Debug, Deserialize)]
struct Evidence { id: String, requirement_id: String, kind: String, outcome: String, commit: String, source: String }
#[derive(Debug, Deserialize)]
struct PolicyFile { rules: Vec<Rule> }
#[derive(Debug, Deserialize)]
struct Rule { requirement_id: String, required_kinds: Vec<String>, #[serde(default)] human_review: bool }
#[derive(Debug, Serialize)]
struct Assessment { requirement_id: String, status: &'static str, rationale: String, evidence_ids: Vec<String> }
#[derive(Debug, Serialize)]
struct Report { application: String, commit: String, catalogue_version: String, assessments: Vec<Assessment> }
fn parse<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T> {
    serde_json::from_str(&fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?)
        .with_context(|| format!("parsing {}", path.display()))
}
fn validate(c: &Catalogue) -> Result<()> {
    anyhow::ensure!(!c.version.is_empty() && !c.requirements.is_empty(), "empty catalogue");
    let mut ids = HashSet::new();
    for r in &c.requirements {
        anyhow::ensure!(!r.id.is_empty() && !r.text.is_empty() && (1..=3).contains(&r.level), "invalid requirement: {}", r.id);
        anyhow::ensure!(ids.insert(&r.id), "duplicate requirement: {}", r.id);
    }
    Ok(())
}
fn assess(c: &Catalogue, e: &EvidenceFile, p: &PolicyFile) -> Result<Report> {
    validate(c)?;
    anyhow::ensure!(!e.application.is_empty() && !e.commit.is_empty(), "application and commit required");
    let rules: BTreeMap<_, _> = p.rules.iter().map(|r| (r.requirement_id.as_str(), r)).collect();
    anyhow::ensure!(rules.len() == p.rules.len(), "duplicate policy rule");
    let valid_ids: HashSet<_> = c.requirements.iter().map(|r| r.id.as_str()).collect();
    for rule in &p.rules { anyhow::ensure!(valid_ids.contains(rule.requirement_id.as_str()), "policy references unknown requirement {}", rule.requirement_id); }
    let mut assessments = Vec::new();
    for r in &c.requirements {
        let relevant: Vec<_> = e.evidence.iter().filter(|x| x.requirement_id == r.id && x.commit == e.commit).collect();
        let ids: Vec<_> = relevant.iter().map(|x| x.id.clone()).collect();
        let (status, rationale) = match rules.get(r.id.as_str()) {
            None => ("NO EVIDENCE", "No verified assessment policy defined; catalogue presence is not compliance".to_string()),
            Some(_) if relevant.is_empty() => ("NO EVIDENCE", "No evidence for the assessed commit".to_string()),
            Some(_) if relevant.iter().any(|x| x.outcome == "failed") => ("FAIL", "A current evidence record reports failure".to_string()),
            Some(rule) if rule.human_review => ("NEEDS REVIEW", "Policy requires independent human verification".to_string()),
            Some(rule) if rule.required_kinds.iter().all(|k| relevant.iter().any(|x| &x.kind == k && x.outcome == "passed")) => ("PASS", "All explicitly required evidence kinds passed for the assessed commit (PoC only)".to_string()),
            Some(_) => ("NEEDS REVIEW", "Evidence incomplete, unrecognised or does not satisfy the declared rule".to_string()),
        };
        assessments.push(Assessment { requirement_id: r.id.clone(), status, rationale, evidence_ids: ids });
    }
    Ok(Report { application: e.application.clone(), commit: e.commit.clone(), catalogue_version: c.version.clone(), assessments })
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Catalogue { input } => {
            let c: Catalogue = parse(&input)?; validate(&c)?;
            println!("Validated {} requirements from {} (PoC schema)", c.requirements.len(), c.version);
        }
        Command::Assess { catalogue, evidence, policy, output } => {
            let c: Catalogue = parse(&catalogue)?;
            let e: EvidenceFile = parse(&evidence)?;
            let p: PolicyFile = parse(&policy)?;
            let report = assess(&c, &e, &p)?;
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(path) = output { fs::write(&path, json)?; println!("Report written to {}", path.display()); }
            else { println!("{json}"); }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_has_pass_fail_review_and_missing() {
        let c: Catalogue = serde_json::from_str(include_str!("../fixtures/catalogue.json")).unwrap();
        let e: EvidenceFile = serde_json::from_str(include_str!("../fixtures/evidence.json")).unwrap();
        let p: PolicyFile = serde_json::from_str(include_str!("../fixtures/policy.json")).unwrap();
        let report = assess(&c, &e, &p).unwrap();
        let statuses: Vec<_> = report.assessments.iter().map(|a| a.status).collect();
        assert_eq!(statuses, vec!["PASS", "FAIL", "NEEDS REVIEW", "NO EVIDENCE"]);
    }
    #[test]
    fn stale_commit_does_not_pass() {
        let c: Catalogue = serde_json::from_str(include_str!("../fixtures/catalogue.json")).unwrap();
        let mut e: EvidenceFile = serde_json::from_str(include_str!("../fixtures/evidence.json")).unwrap();
        e.commit = "different-commit".into();
        let p: PolicyFile = serde_json::from_str(include_str!("../fixtures/policy.json")).unwrap();
        assert!(assess(&c, &e, &p).unwrap().assessments.iter().all(|a| a.status == "NO EVIDENCE"));
    }
}
