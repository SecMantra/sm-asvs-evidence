//! cargo run --example consume
use sm_asvs_evidence::{assess, Catalogue, EvidenceFile, PolicyFile};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalogue: Catalogue = serde_json::from_str(include_str!("../fixtures/catalogue.json"))?;
    let evidence: EvidenceFile = serde_json::from_str(include_str!("../fixtures/evidence.json"))?;
    let policy: PolicyFile = serde_json::from_str(include_str!("../fixtures/policy.json"))?;
    let report = assess(&catalogue, &evidence, &policy)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
