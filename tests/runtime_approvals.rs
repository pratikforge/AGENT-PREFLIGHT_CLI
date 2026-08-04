use std::time::SystemTime;
use tempfile::TempDir;

use agent_preflight::app::runtime::{
    InterceptDecision, RuntimeApproval, RuntimeInterceptor, ToolInvocation,
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

#[test]
fn require_approval_for_sensitive_action() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let invocation = ToolInvocation {
        tool_name: "bash".to_string(),
        args: serde_json::json!(["-c", "rm -rf /"]),
    };

    let decision = interceptor.intercept(&invocation);
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );

    let audit_log = std::fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(audit_log.contains("REDACTED"));
}

#[test]
fn grant_approval_successfully() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let invocation = ToolInvocation {
        tool_name: "bash".to_string(),
        args: serde_json::json!(["-c", "rm -rf /"]),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test".to_string(),
        context_hash: "abcd".to_string(),
        expires_at: now + 3600,
        nonce: "1234".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&invocation, Some(&approval));
    assert_eq!(decision, InterceptDecision::Allow);
}

#[test]
fn reject_expired_approval() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let invocation = ToolInvocation {
        tool_name: "bash".to_string(),
        args: serde_json::json!(["-c", "rm -rf /"]),
    };

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let approval = RuntimeApproval {
        rule_id: "unsafe-rm-rf".to_string(),
        caller_id: "test".to_string(),
        context_hash: "abcd".to_string(),
        expires_at: now - 3600,
        nonce: "1234".to_string(),
    };

    let decision = interceptor.intercept_with_approval(&invocation, Some(&approval));
    assert_eq!(
        decision,
        InterceptDecision::RequireApproval("unsafe-rm-rf".to_string())
    );
}
