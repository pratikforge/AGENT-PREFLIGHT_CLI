use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "agent-preflight",
    version,
    about = "Static preflight checks for agent repositories"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Scan {
        repository_path: PathBuf,
    },
    Review {
        repository_path: PathBuf,
    },
    Approve {
        repository_path: PathBuf,
        rule_id: String,
    },
    Task {
        repository_path: PathBuf,
        rule_id: String,
    },
    Verify {
        repository_path: PathBuf,
        #[arg(long)]
        ci: bool,
    },
}
