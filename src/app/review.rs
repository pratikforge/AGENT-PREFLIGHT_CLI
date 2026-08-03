use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::Path;

use crate::domain::contract::Contract;

pub fn run(root: &Path) -> Result<String, ReviewError> {
    let path = root.join(".agent-preflight").join("contract.proposed.yaml");
    let input = fs::read_to_string(path).map_err(|_| ReviewError::MissingProposal)?;
    let contract = Contract::from_yaml(&input).map_err(|_| ReviewError::InvalidProposal)?;
    let rules = contract
        .rules
        .iter()
        .map(|rule| format!("- `{}`: {}\n", rule.id, rule.intended_capability))
        .collect::<String>();

    let is_interactive =
        io::stdin().is_terminal() || std::env::var("AGENT_PREFLIGHT_FORCE_INTERACTIVE").is_ok();

    let pending_message = format!(
        "# Pending capability rules\n\nRevision: `{}`\n\n{rules}Approve one rule explicitly with `agent-preflight approve <repository-path> <rule-id>`.\n",
        contract.revision_sha256
    );

    if !is_interactive {
        return Ok(pending_message);
    }

    println!("{pending_message}");
    for rule in &contract.rules {
        print!("Approve rule '{}'? (Y/n) ", rule.id);
        io::stdout().flush().ok();
        let mut response = String::new();
        match io::stdin().read_line(&mut response) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let answer = response.trim();
                if answer == "Y" || answer == "y" {
                    if let Err(e) = crate::app::approve::run(root, &rule.id) {
                        eprintln!("Failed to approve {}: {}", rule.id, e);
                    } else {
                        println!("Approved `{}`", rule.id);
                        // Contract can only hold one approved rule per run (per current design)
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }

    // Return empty string since we already printed the message interactively
    Ok(String::new())
}

#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("no proposed contract exists; run scan first")]
    MissingProposal,
    #[error("proposed contract is invalid")]
    InvalidProposal,
}
