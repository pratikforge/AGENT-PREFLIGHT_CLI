use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use console::{Term, style};
use dialoguer::{MultiSelect, Select, theme::ColorfulTheme};

use crate::domain::contract::Contract;
use crate::infra::opener;

pub const MENU_SELECTIONS: &[&str] = &[
    "► Scan Current Directory",
    "► View Report",
    "► Review & Approve Rules",
    "► Verify (CI Gate)",
];

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn run_interactive_menu() {
    let term = Term::stderr();
    term.clear_screen().ok();

    print_banner();

    let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());

    let theme = ColorfulTheme {
        prompt_prefix: style("›".to_string()).for_stderr().cyan().bold(),
        success_prefix: style("›".to_string()).for_stderr().green().bold(),
        ..ColorfulTheme::default()
    };

    loop {
        println!();
        let selection = Select::with_theme(&theme)
            .with_prompt(format!("{}", style("Select an action").bold()))
            .default(0)
            .items(MENU_SELECTIONS)
            .interact_opt();

        match selection {
            Ok(Some(0)) => handle_scan(&current_dir),
            Ok(Some(1)) => handle_view_report(&current_dir),
            Ok(Some(2)) => handle_review_approve(&current_dir, &theme),
            Ok(Some(3)) => handle_verify(&current_dir),
            Ok(None) | Err(_) => break,
            _ => unreachable!(),
        }

        println!(
            "\n{}",
            style("───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────")
                .dim()
        );
    }
}

fn print_banner() {
    let border_h = "═".repeat(119);
    let empty_line = format!("║{}║", " ".repeat(119));

    println!("{}", style(format!("╔{border_h}╗")).white().bold());
    println!("{}", style(&empty_line).white().bold());

    let art_lines = [
        r"  █████╗  ██████╗ ███████╗███╗   ██╗████████╗    ██████╗ ██████╗ ███████╗███████╗██╗     ██╗ ██████╗ ██╗  ██╗████████╗ ",
        r" ██╔══██╗██╔════╝ ██╔════╝████╗  ██║╚══██╔══╝    ██╔══██╗██╔══██╗██╔════╝██╔════╝██║     ██║██╔════╝ ██║  ██║╚══██╔══╝ ",
        r" ███████║██║  ███╗█████╗  ██╔██╗ ██║   ██║       ██████╔╝██████╔╝█████╗  █████╗  ██║     ██║██║  ███╗███████║   ██║    ",
        r" ██╔══██║██║   ██║██╔══╝  ██║╚═╝██║   ██║       ██╔═══╝ ██╔══██╗██╔══╝  ██╔══╝  ██║     ██║██║   ██║██╔══██║   ██║    ",
        r" ██║  ██║╚██████╔╝███████╗██║ ╚████║   ██║       ██║     ██║  ██║███████╗██║     ███████╗██║╚██████╔╝██║  ██║   ██║    ",
        r" ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝   ╚═╝       ╚═╝     ╚═╝  ╚═╝╚══════╝╚═╝     ╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ",
    ];

    for line in &art_lines {
        println!("{}", style(format!("║{line}║")).white().bold());
    }

    println!("{}", style(&empty_line).white().bold());

    // Tagline, centered
    let tagline = "Static verification & runtime protection for AI agents";
    let pad2 = (119 - tagline.len()) / 2;
    let pad2_r = 119 - pad2 - tagline.len();
    let tagline_line = format!("║{}{}{}║", " ".repeat(pad2), tagline, " ".repeat(pad2_r));
    println!("{}", style(tagline_line).white().bold());

    println!("{}", style(&empty_line).white().bold());
    println!("{}", style(format!("╚{border_h}╝")).white().bold());
}

/// Animate a spinner while the given closure runs, then print a done message.
fn with_spinner<F, T>(message: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let msg = message.to_string();
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = thread::spawn(move || {
        let mut i = 0;
        loop {
            if rx.try_recv().is_ok() {
                // Clear the spinner line
                eprint!("\r{}\r", " ".repeat(msg.len() + 10));
                io::stderr().flush().ok();
                return;
            }
            eprint!(
                "\r  {} {}",
                style(SPINNER_FRAMES[i % SPINNER_FRAMES.len()])
                    .cyan()
                    .bold(),
                style(&msg).dim()
            );
            io::stderr().flush().ok();
            i += 1;
            thread::sleep(Duration::from_millis(80));
        }
    });

    let result = f();
    tx.send(()).ok();
    handle.join().ok();
    result
}

fn print_section_header(title: &str) {
    println!("\n  {}", style(title).bold().underlined());
    println!();
}

fn print_success(msg: &str) {
    println!("  {} {}", style("[OK]").green().bold(), style(msg).bold());
}

fn print_warning(msg: &str) {
    println!("  {} {}", style("[!!]").yellow().bold(), style(msg).bold());
}

fn print_error(msg: &str) {
    eprintln!("  {} {}", style("[ERR]").red().bold(), style(msg).bold());
}

fn print_info(label: &str, value: &str) {
    println!("  {}  {}", style(format!("{label}:")).dim(), value);
}

fn handle_scan(root: &Path) {
    print_section_header("SCAN");

    let result = with_spinner(
        &format!("Scanning repository at {} ...", root.display()),
        || crate::app::scan::run(root),
    );

    match result {
        Ok(result) => {
            print_success("Scan completed successfully");
            println!();
            print_info("Profile", result.profile.label());
            print_info("Files scanned", &result.files_scanned.to_string());

            if result.has_parse_error {
                println!();
                print_warning("Some files had parse errors");
            }
        }
        Err(e) => print_error(&format!("Scan failed: {e}")),
    }
}

fn handle_view_report(root: &Path) {
    print_section_header("REPORT");

    let report_path = root.join(".agent-preflight").join("report.md");
    if !report_path.exists() {
        print_error("No report found — run a scan first");
        return;
    }

    println!("  Opening report in your editor...");

    if let Err(e) = opener::open_file(&report_path) {
        print_error(&format!("Could not open report: {e}"));
        print_info("Manual path", &report_path.display().to_string());
    } else {
        print_success("Report opened");
    }
}

fn handle_review_approve(root: &Path, theme: &ColorfulTheme) {
    print_section_header("REVIEW & APPROVE");

    let path = root.join(".agent-preflight").join("contract.proposed.yaml");
    let input = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => {
            print_error("No proposed contract found — run a scan first");
            return;
        }
    };

    let contract = match Contract::from_yaml(&input) {
        Ok(c) => c,
        Err(_) => {
            print_error("Proposed contract is invalid");
            return;
        }
    };

    if contract.rules.is_empty() {
        print_success("No pending rules to approve");
        return;
    }

    print_info("Revision", &contract.revision_sha256);
    println!("  {}  {}\n", style("Pending:").dim(), contract.rules.len());

    let items: Vec<String> = contract
        .rules
        .iter()
        .map(|rule| {
            format!(
                "{}  —  {}",
                style(&rule.id).bold(),
                rule.intended_capability
            )
        })
        .collect();

    let selection = MultiSelect::with_theme(theme)
        .with_prompt(format!(
            "{}",
            style("Toggle with SPACE, confirm with ENTER").bold()
        ))
        .items(&items)
        .interact_opt();

    println!();

    match selection {
        Ok(Some(selected_indices)) => {
            if selected_indices.is_empty() {
                print_warning("No rules selected");
                return;
            }

            for idx in selected_indices {
                let rule = &contract.rules[idx];
                match crate::app::approve::run(root, &rule.id) {
                    Ok(()) => print_success(&format!("Approved  {}", rule.id)),
                    Err(e) => print_error(&format!("Failed to approve {}: {}", rule.id, e)),
                }
            }
        }
        Ok(None) | Err(_) => {
            print_warning("Approval cancelled");
        }
    }
}

fn handle_verify(root: &Path) {
    print_section_header("VERIFY");

    let result = with_spinner("Running CI verification ...", || {
        crate::app::verify::run(root)
    });

    match result {
        Ok(result) => {
            let status = result.status;
            let code = crate::app::verify::ci_exit_code(status);

            if code == 0 {
                print_success(&format!("Verified  (Exit Code: {code})"));
            } else {
                print_warning(&format!("Status: {status:?}  (Exit Code: {code})"));
            }
        }
        Err(e) => print_error(&format!("Verification failed: {e}")),
    }
}
