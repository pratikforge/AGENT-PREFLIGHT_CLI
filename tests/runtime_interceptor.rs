use std::path::Path;
use tempfile::TempDir;

use agent_preflight::app::runtime::{InterceptDecision, RuntimeInterceptor, ToolInvocation};
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
fn denied_fake_tool_records_execution() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "deny".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let invocation = ToolInvocation {
        tool_name: "bash".to_string(),
        args: serde_json::json!(["-c", "rm -rf /"]),
    };

    let decision = interceptor.intercept(&invocation);
    assert_eq!(
        decision,
        InterceptDecision::Deny("unsafe-rm-rf".to_string())
    );

    let audit_log = std::fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(audit_log.contains("unsafe-rm-rf"));
    assert!(audit_log.contains("Deny"));
}

#[test]
fn allowed_tool_does_not_execute() {
    // We expect it to return Allow for allowed tool
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "proposed; owner review required".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let invocation = ToolInvocation {
        tool_name: "bash".to_string(),
        args: serde_json::json!(["-c", "echo hello"]),
    };

    let decision = interceptor.intercept(&invocation);
    assert_eq!(decision, InterceptDecision::Allow);
}

#[test]
fn approval_required_tool_executes_before_approval() {
    // For a tool that needs approval, it returns RequireApproval
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
}
