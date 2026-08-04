use agent_preflight::domain::evidence::EvidenceRef;
use agent_preflight::domain::ir::{
    Agent, CapabilityIr, Dependency, EvidenceNode, McpServer, NetworkDestination, Sandbox,
    SensitiveData, Tool,
};

#[test]
fn serialize_deserialize_an_agent_with_entities() {
    let ir = CapabilityIr {
        agents: vec![Agent {
            id: "agent_1".to_string(),
            provider: "openai".to_string(),
            tools: vec![Tool {
                id: "tool_1".to_string(),
                implementation: "shell".to_string(),
                approval_control: "always".to_string(),
            }],
            mcp_servers: vec![McpServer {
                endpoint: "http://localhost".to_string(),
                transport: "http".to_string(),
            }],
            sandbox: Some(Sandbox {
                network: "restricted".to_string(),
            }),
            destinations: vec![NetworkDestination {
                hostname: "example.com".to_string(),
            }],
            sensitive_data: vec![SensitiveData {
                classification: "pii".to_string(),
            }],
            dependencies: vec![Dependency {
                package: "requests".to_string(),
            }],
            evidence: EvidenceNode {
                origin: "test.py".to_string(),
                refs: vec![EvidenceRef {
                    path: "test.py".to_string(),
                    line: 1,
                    parser_error: false,
                }],
            },
        }],
        edges: vec![],
    };

    let serialized = serde_json::to_string(&ir).unwrap();
    let deserialized: CapabilityIr = serde_json::from_str(&serialized).unwrap();
    assert_eq!(ir, deserialized);
}

#[test]
fn reject_an_evidence_node_without_origin() {
    let json = r#"{
        "origin": "",
        "refs": []
    }"#;
    assert!(
        serde_json::from_str::<EvidenceNode>(json)
            .unwrap()
            .validate()
            .is_err()
    );
}
