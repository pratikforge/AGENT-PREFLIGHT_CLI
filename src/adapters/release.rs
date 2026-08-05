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

pub fn evaluate(yaml: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let doc = match serde_yaml_ng::from_str::<Value>(yaml) {
        Ok(v) => v,
        Err(_) => return findings,
    };

    let evidence = EvidenceRef {
        path: "repo_config.yml".to_string(),
        line: 0,
        parser_error: false,
    };

    if let Some(branches) = doc.get("branches").and_then(|v| v.as_mapping()) {
        if let Some(main) = branches
            .get(Value::String("main".to_string()))
            .and_then(|v| v.as_mapping())
        {
            if main.get(Value::String("require_signed_commits".to_string()))
                == Some(&Value::Bool(true))
            {
                findings.push(Finding {
                    rule_id: "rejects_unsigned_commits_in_protected_branches".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "RELEASE".to_string(),
                });
            }
        }
    }

    if let Some(rules) = doc.get("rules").and_then(|v| v.as_mapping()) {
        if let Some(policy) = rules
            .get(Value::String("policy".to_string()))
            .and_then(|v| v.as_mapping())
        {
            if policy.get(Value::String("min_approvers".to_string()))
                == Some(&Value::Number(2.into()))
            {
                findings.push(Finding {
                    rule_id: "requires_two_party_review_for_policy_changes".to_string(),
                    status: Status::Verified,
                    evidence: evidence.clone(),
                    matrix_source: "RELEASE".to_string(),
                });
            }
        }
    }

    if let Some(builds) = doc.get("builds").and_then(|v| v.as_mapping()) {
        if builds.get(Value::String("require_provenance".to_string())) == Some(&Value::Bool(true)) {
            findings.push(Finding {
                rule_id: "verifies_provenance_attestation_for_all_builds".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "RELEASE".to_string(),
            });
        }
    }

    if let Some(release) = doc.get("release").and_then(|v| v.as_mapping()) {
        if release.get(Value::String("require_quality_gate".to_string()))
            == Some(&Value::Bool(true))
        {
            findings.push(Finding {
                rule_id: "denies_release_without_successful_quality_gate".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "RELEASE".to_string(),
            });
        }
    }

    if let Some(tags) = doc.get("tags").and_then(|v| v.as_mapping()) {
        if tags.get(Value::String("immutable".to_string())) == Some(&Value::Bool(true)) {
            findings.push(Finding {
                rule_id: "enforces_immutable_tags_for_releases".to_string(),
                status: Status::Verified,
                evidence: evidence.clone(),
                matrix_source: "RELEASE".to_string(),
            });
        }
    }

    findings
}
