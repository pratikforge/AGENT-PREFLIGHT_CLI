#![allow(clippy::collapsible_if)]
use crate::domain::evidence::EvidenceRef;
use crate::domain::status::Status;
use serde_yaml_ng::Value;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(yaml: &str, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let doc = match serde_yaml_ng::from_str::<Value>(yaml) {
        Ok(v) => v,
        Err(_) => return findings,
    };

    let evidence = EvidenceRef {
        path: path.to_string(),
        line: 0,
        parser_error: false,
    };

    let mut has_clippy = false;
    let mut has_fmt = false;
    let mut has_adapter_check = false;
    let mut has_license_check = false;
    let mut has_audit_test = false;
    let mut has_approval_env = false;
    let mut has_coverage_check = false;

    if let Some(env) = doc.get("env").and_then(|v| v.as_mapping()) {
        if env.contains_key(Value::String("REQUIRE_APPROVAL".to_string())) {
            has_approval_env = true;
        }
    }

    if let Some(jobs) = doc.get("jobs").and_then(|v| v.as_mapping()) {
        for (_job_name, job) in jobs {
            if let Some(steps) = job.get("steps").and_then(|v| v.as_sequence()) {
                for step in steps {
                    if let Some(run) = step.get("run").and_then(|v| v.as_str()) {
                        if run.contains("cargo clippy") && run.contains("--all-targets") {
                            has_clippy = true;
                        }
                        if run.contains("cargo fmt") && run.contains("--check") {
                            has_fmt = true;
                        }
                        if run.contains("check_adapter_tests.sh") {
                            has_adapter_check = true;
                        }
                        if run.contains("cargo deny check licenses") {
                            has_license_check = true;
                        }
                        if run.contains("cargo test") && run.contains("audit_layer") {
                            has_audit_test = true;
                        }
                        if run.contains("cargo tarpaulin") && run.contains("--fail-under") {
                            has_coverage_check = true;
                        }
                    }
                }
            }
        }
    }

    if has_clippy {
        findings.push(Finding {
            rule_id: "runs_clippy_on_all_targets".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_fmt {
        findings.push(Finding {
            rule_id: "runs_format_check_before_build".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_adapter_check {
        findings.push(Finding {
            rule_id: "denies_new_untested_adapter_rules".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_license_check {
        findings.push(Finding {
            rule_id: "fails_ci_if_dependency_licenses_unapproved".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_audit_test {
        findings.push(Finding {
            rule_id: "requires_audit_layer_coverage".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_approval_env {
        findings.push(Finding {
            rule_id: "prevents_bypassing_approval_in_ci_environment".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if has_coverage_check {
        findings.push(Finding {
            rule_id: "fails_ci_if_test_coverage_drops".to_string(),
            status: Status::Verified,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if let Some(perms) = doc.get("permissions").and_then(|v| v.as_str()) {
        if perms == "write-all" {
            findings.push(Finding {
                rule_id: "ci-permissions-write-all".to_string(),
                status: Status::Failed,
                evidence: evidence.clone(),
                matrix_source: "CI_POSTURE".to_string(),
            });
        }
    }

    if let Some(perms) = doc.get("permissions").and_then(|v| v.as_mapping()) {
        if !perms.contains_key(Value::String("id-token".to_string())) {
            findings.push(Finding {
                rule_id: "ci-missing-oidc".to_string(),
                status: Status::Failed,
                evidence: evidence.clone(),
                matrix_source: "CI_POSTURE".to_string(),
            });
        }
    } else {
        findings.push(Finding {
            rule_id: "ci-missing-oidc".to_string(),
            status: Status::Failed,
            evidence: evidence.clone(),
            matrix_source: "CI_POSTURE".to_string(),
        });
    }

    if let Some(jobs) = doc.get("jobs").and_then(|v| v.as_mapping()) {
        for (_job_name, job) in jobs {
            if let Some(steps) = job.get("steps").and_then(|v| v.as_sequence()) {
                for step in steps {
                    if let Some(uses) = step.get("uses").and_then(|v| v.as_str())
                        && uses.contains('@')
                        && !uses.contains("@sha256:")
                        && uses.len() < uses.find('@').unwrap() + 40
                    {
                        findings.push(Finding {
                            rule_id: "ci-unpinned-action".to_string(),
                            status: Status::Failed,
                            evidence: evidence.clone(),
                            matrix_source: "CI_POSTURE".to_string(),
                        });
                    }
                }
            }
        }
    }

    findings
}
