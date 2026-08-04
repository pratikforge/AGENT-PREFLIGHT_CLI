use std::fs;
use tempfile::TempDir;

use agent_preflight::app::runtime::{
    AuditLog, InterceptDecision, RuntimeInterceptor, RuntimeRequest,
};
use agent_preflight::domain::contract::{Contract, Rule};

fn create_test_contract(rules: Vec<Rule>) -> Contract {
    Contract {
        schema_version: 1,
        profile: "test".to_string(),
        policy_revision: "v1.0.0".to_string(),
        rules,
        revision_sha256: "".to_string(),
    }
}

// ---------------------------------------------------------
// Mandatory Phase 1.2 Tests
// ---------------------------------------------------------

#[test]
fn never_writes_secret_or_pii_canaries_for_any_decision() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "echo my_secret_token_123"]), // CANARY
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let _ = interceptor.intercept(&req);

    let log_content = fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(!log_content.contains("my_secret_token_123"));
}

#[test]
fn records_stable_request_fingerprint_not_raw_arguments() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "echo hello"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "STABLE-FINGERPRINT-999".to_string(),
    };

    let _ = interceptor.intercept(&req);

    let log_content = fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(log_content.contains("STABLE-FINGERPRINT-999"));
    assert!(!log_content.contains("echo hello"));
}

#[test]
fn fails_closed_when_required_audit_write_fails() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "safe-rule".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "low".to_string(),
        approval_requirement: "none".to_string(),
    }]);

    let mut interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    // Force audit log path to an unwritable location
    interceptor.set_audit_log_path(dir.path().join("non_existent").join("invalid.log"));

    let req = RuntimeRequest {
        capability_id: "safe-command".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["arg"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest".to_string(),
    };

    let decision = interceptor.intercept(&req);
    // Should fail closed because it cannot write the audit log
    match decision {
        InterceptDecision::Deny(msg) => {
            assert!(msg.contains("audit"));
        }
        _ => panic!(
            "Expected Deny due to audit write failure, got {:?}",
            decision
        ),
    }
}

#[test]
fn verifies_newly_written_audit_chain() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]);
    let log_path = dir.path().join("audit.log");

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());

    let req1 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["1"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-1".to_string(),
    };
    let _ = interceptor.intercept(&req1);

    let req2 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["2"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-2".to_string(),
    };
    let _ = interceptor.intercept(&req2);

    let audit = AuditLog::load(&log_path).unwrap();
    assert_eq!(audit.verify_chain(), Ok(true));
}

#[test]
fn detects_modified_middle_audit_record() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]);
    let log_path = dir.path().join("audit.log");

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());

    let req1 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["1"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-1".to_string(),
    };
    let _ = interceptor.intercept(&req1);

    let req2 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["2"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-2".to_string(),
    };
    let _ = interceptor.intercept(&req2);

    // Tamper with the middle of the file
    let content = fs::read_to_string(&log_path).unwrap();
    let tampered = content.replace("digest-1", "digest-TAMPERED");
    fs::write(&log_path, tampered).unwrap();

    let audit = AuditLog::load(&log_path).unwrap();
    assert!(audit.verify_chain().is_err());
}

#[test]
fn detects_deleted_or_reordered_audit_record() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]);
    let log_path = dir.path().join("audit.log");

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());

    let req1 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["1"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-1".to_string(),
    };
    let _ = interceptor.intercept(&req1);

    let req2 = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["2"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-2".to_string(),
    };
    let _ = interceptor.intercept(&req2);

    // Delete first line
    let content = fs::read_to_string(&log_path).unwrap();
    let mut lines: Vec<&str> = content.lines().collect();
    lines.remove(0); // Delete first record
    let tampered = lines.join("\n") + "\n";
    fs::write(&log_path, tampered).unwrap();

    let audit = AuditLog::load(&log_path).unwrap();
    assert!(audit.verify_chain().is_err());
}
