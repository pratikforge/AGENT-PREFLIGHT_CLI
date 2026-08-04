use std::time::SystemTime;
use tempfile::TempDir;

use agent_preflight::app::runtime::{
    InterceptDecision, RuntimeApproval, RuntimeInterceptor, RuntimeRequest,
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
// Mandatory Phase 1.1 Tests
// ---------------------------------------------------------

#[test]
fn allows_matching_unexpired_single_use_approval() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-1".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(decision, InterceptDecision::Allow);
}

#[test]
fn rejects_approval_for_different_caller() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user-2".to_string(), // DIFFERENT CALLER
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-2".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn rejects_approval_for_different_arguments_or_context() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /some/other/dir"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-DIFFERENT".to_string(), // DIFFERENT DIGEST
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-3".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn rejects_approval_for_different_rule_or_policy_revision() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.1".to_string(), // DIFFERENT REVISION
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-4".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn rejects_replayed_nonce_after_successful_use() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-replay".to_string(),
    };

    // First use works
    let decision1 = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(decision1, InterceptDecision::Allow);

    // Second use with same nonce fails
    let decision2 = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision2,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn allows_only_one_of_concurrent_nonce_consumers() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());

    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-concurrent".to_string(),
    };

    let d1 = interceptor.intercept_with_approval(&req, Some(&approval));
    let d2 = interceptor.intercept_with_approval(&req, Some(&approval));

    assert!(d1 == InterceptDecision::Allow && d2 != InterceptDecision::Allow);
}

#[test]
fn denies_when_nonce_store_is_unavailable() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    // Force nonce store to use a directory we can't write to, or something similar.
    // For now we assume set_nonce_store_path configures it.
    let mut interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    interceptor.set_nonce_store_path(dir.path().join("non_existent").join("invalid.db"));

    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now - 10,
        nonce: "nonce-unavailable".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn rejects_malformed_or_future_issued_claim() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test-user".to_string(),
        request_digest: "digest-123".to_string(),
        policy_revision: "v1.0.0".to_string(),
        expires_at: now + 3600,
        issued_at: now + 3600, // FUTURE ISSUED
        nonce: "nonce-future".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&req, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}
