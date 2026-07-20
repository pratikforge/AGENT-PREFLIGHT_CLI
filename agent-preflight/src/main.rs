//! Command surface for Agent Preflight.
//!
//! T01 intentionally declares commands only. The handlers do not inspect a
//! repository, create artifacts, or call external services.

use clap::Parser;

use agent_preflight::app::{approve, review, scan, task, verify};
use agent_preflight::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Scan { repository_path }) => match scan::run(&repository_path) {
            Ok(result) => {
                println!(
                    "scan complete: {} ({} files)",
                    result.profile.label(),
                    result.files_scanned
                );
                if result.has_parse_error {
                    std::process::exit(4);
                }
                if result.profile == agent_preflight::adapters::Profile::Unsupported {
                    std::process::exit(3);
                }
            }
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        },
        Some(Command::Review { repository_path }) => match review::run(&repository_path) {
            Ok(output) => print!("{output}"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        },
        Some(Command::Approve {
            repository_path,
            rule_id,
        }) => match approve::run(&repository_path, &rule_id) {
            Ok(()) => println!("approved `{rule_id}`"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        },
        Some(Command::Task {
            repository_path,
            rule_id,
        }) => match task::run(&repository_path, &rule_id) {
            Ok(()) => println!("repair packet generated for `{rule_id}`"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        },
        Some(Command::Verify {
            repository_path,
            ci: _,
        }) => match verify::run(&repository_path) {
            Ok(result) => std::process::exit(verify::ci_exit_code(result.status)),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(2);
            }
        },
        None => {}
    }
}
