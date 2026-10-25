use anyhow::{Context, Result};
use sm_asvs_evidence::{assess, Catalogue, EvidenceFile, PolicyFile};
use clap::{Parser, Subcommand};
use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "sm-asvs-evidence", version, about = "Offline evidence assessment (experimental)")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Catalogue {
        #[arg(long)]
        input: PathBuf,
    },
    Assess {
        #[arg(long)]
        catalogue: PathBuf,
        #[arg(long)]
        evidence: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn load<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Catalogue { input } => {
            let catalogue: Catalogue = load(&input)?;
            // Validate using the same library rules as consumers.
            let empty_evidence = EvidenceFile {
                application: "catalogue-validation".into(),
                commit: "validation".into(),
                evidence: Vec::new(),
            };
            let empty_policy = PolicyFile { rules: Vec::new() };
            sm_asvs_evidence::validate(&catalogue, &empty_evidence, &empty_policy)
                .map_err(anyhow::Error::msg)?;
            println!("Validated {} requirements from {}", catalogue.requirements.len(), catalogue.version);
        }
        Command::Assess { catalogue, evidence, policy, output } => {
            let catalogue: Catalogue = load(&catalogue)?;
            let evidence: EvidenceFile = load(&evidence)?;
            let policy: PolicyFile = load(&policy)?;
            let report = assess(&catalogue, &evidence, &policy).map_err(anyhow::Error::msg)?;
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(path) = output {
                fs::write(&path, json)?;
                println!("Report written to {}", path.display());
            } else {
                println!("{json}");
            }
        }
    }
    Ok(())
}
