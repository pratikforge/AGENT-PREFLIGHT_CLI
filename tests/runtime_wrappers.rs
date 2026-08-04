use std::sync::{Arc, Mutex};
use tempfile::TempDir;

use agent_preflight::app::runtime::{RuntimeApproval, RuntimeInterceptor, RuntimeRequest};
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

// RuntimeGuard is now in src/app/runtime.rs
use agent_preflight::app::runtime::RuntimeGuard;

// ----------------------------------------------------------------
// Mandatory Phase 2.1 Tests
// ---------------------------------------------------------

#[test]
fn denied_request_never_calls_fake_executor() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "deny".to_string(), // DENY
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let result = guard.execute(&req, None, || {
        *called_clone.lock().unwrap() = true;
        Ok(())
    });

    assert!(result.is_err());
    assert_eq!(*called.lock().unwrap(), false);
}

#[test]
fn approval_pending_request_never_calls_fake_executor() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    // No approval provided
    let result = guard.execute(&req, None, || {
        *called_clone.lock().unwrap() = true;
        Ok(())
    });

    assert!(result.is_err());
    assert_eq!(*called.lock().unwrap(), false);
}

#[test]
fn valid_approved_request_calls_fake_executor_once() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-rm-rf".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let now = std::time::SystemTime::now()
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
        nonce: "nonce-guard-1".to_string(),
    };

    let called = Arc::new(Mutex::new(0));
    let called_clone = called.clone();

    let result = guard.execute(&req, Some(&approval), || {
        *called_clone.lock().unwrap() += 1;
        Ok(())
    });

    assert!(result.is_ok());
    assert_eq!(*called.lock().unwrap(), 1);
}

#[test]
fn missing_contract_denies_before_executor() {
    let dir = TempDir::new().unwrap();
    // Missing contract
    let interceptor = RuntimeInterceptor::new(None, dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let result = guard.execute(&req, None, || {
        *called_clone.lock().unwrap() = true;
        Ok(())
    });

    assert!(result.is_err());
    assert_eq!(*called.lock().unwrap(), false);
}

#[test]
fn unrecognized_sensitive_capability_is_not_silently_allowed() {
    // This implies that if a capability is unrecognized but matches a heuristic, it should be denied.
    // Our logic currently allows unrecognized capabilities unless they match a specific rule regex.
    // Wait, the test says "unrecognized sensitive capability is not silently allowed"
    // We can simulate an unrecognized sensitive command like `rm -rf` when there is no contract rule for it.
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![]); // Empty rules

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "bash".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["-c", "rm -rf /"]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    let result = guard.execute(&req, None, || {
        *called_clone.lock().unwrap() = true;
        Ok(())
    });

    assert!(result.is_err());
    assert_eq!(*called.lock().unwrap(), false);
}

#[test]
fn executor_failure_is_audited_without_argument_leakage() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "safe".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "low".to_string(),
        approval_requirement: "none".to_string(), // ALLOW
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let req = RuntimeRequest {
        capability_id: "safe-command".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!(["my_secret_args_123"]), // SECRET
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-safe".to_string(),
    };

    let result: Result<(), String> =
        guard.execute(&req, None, || Err("Executor failed internally".to_string()));

    assert!(result.is_err());

    // Check audit log
    let log_content = std::fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(!log_content.contains("my_secret_args_123"));
}

use agent_preflight::app::runtime::{ADKGeminiWrapper, ClaudeWrapper, MCPWrapper, OpenAIWrapper};

#[test]
fn openai_wrapper_blocks_shell_before_transport() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "unsafe-shell".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "deny".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = OpenAIWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "unsafe-shell".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_err());
}

#[test]
fn adk_gemini_wrapper_requires_approval_before_code_execution() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "code-exec".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = ADKGeminiWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "code-exec".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_err());
}

#[test]
fn claude_wrapper_blocks_disallowed_file_write_before_executor() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "file-write".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "deny".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = ClaudeWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "file-write".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_err());
}

#[test]
fn mcp_wrapper_blocks_unapproved_http_before_connect() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "http-connect".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "high".to_string(),
        approval_requirement: "runtime".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = MCPWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "http-connect".to_string(),
        caller_id: "test-user".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-123".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_err());
}

#[test]
fn wrappers_forward_caller_and_canonical_digest() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "safe-tool".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "low".to_string(),
        approval_requirement: "none".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = OpenAIWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "safe-tool".to_string(),
        caller_id: "alice".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "v1.0.0".to_string(),
        request_digest: "digest-xyz".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_ok());

    let log_content = std::fs::read_to_string(dir.path().join("audit.log")).unwrap();
    assert!(log_content.contains("digest-xyz"));
}

#[test]
fn wrapper_policy_revision_mismatch_fails_closed() {
    let dir = TempDir::new().unwrap();
    let contract = create_test_contract(vec![Rule {
        id: "safe-tool".to_string(),
        intended_capability: "Test".to_string(),
        risk_tier: "low".to_string(),
        approval_requirement: "none".to_string(),
    }]);

    let interceptor = RuntimeInterceptor::new(Some(contract), dir.path());
    let guard = RuntimeGuard::new(interceptor);
    let wrapper = OpenAIWrapper::new(&guard);

    let req = RuntimeRequest {
        capability_id: "safe-tool".to_string(),
        caller_id: "alice".to_string(),
        arguments: serde_json::json!([]),
        policy_revision: "".to_string(), // Empty or mismatch
        request_digest: "digest-xyz".to_string(),
    };

    let result = wrapper.execute_tool(&req, None, || Ok(()));
    assert!(result.is_err());
}
