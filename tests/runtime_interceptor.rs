use tempfile::TempDir;

use agent_preflight::app::runtime::{InterceptDecision, RuntimeInterceptor, RuntimeRequest};
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

#[test]
fn approval_required_tool_executes_before_approval() {
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
        caller_id: "test".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest".to_string(),
    };

    let decision = interceptor.intercept(&req);
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}

#[test]
fn allowed_tool_does_not_execute() {
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
        caller_id: "test".to_string(),
        arguments: serde_json::json!(["-c", "ls -l"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest".to_string(),
    };

    let decision = interceptor.intercept(&req);
    assert_eq!(decision, InterceptDecision::Allow);
}

#[test]
fn denied_fake_tool_records_execution() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-git-force-push".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "deny".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let req = RuntimeRequest {
        capability_id: "git".to_string(),
        caller_id: "test".to_string(),
        arguments: serde_json::json!(["push --force"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest".to_string(),
    };

    let decision = interceptor.intercept(&req);
    assert_eq!(
        decision,
        InterceptDecision::Deny("unsafe-git-force-push".to_string())
    );
}
