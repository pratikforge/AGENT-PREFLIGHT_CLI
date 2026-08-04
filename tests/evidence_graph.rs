use agent_preflight::domain::evidence::EvidenceRef;
use agent_preflight::domain::ir::{CapabilityIr, EvidenceEdge, EvidenceNode};

#[test]
fn attach_source_and_config_evidence_to_one_capability() {
    let source_node = EvidenceNode {
        origin: "src/agent.py".to_string(),
        refs: vec![EvidenceRef {
            path: "src/agent.py".to_string(),
            line: 10,
            parser_error: false,
        }],
    };
    let config_node = EvidenceNode {
        origin: "config.yaml".to_string(),
        refs: vec![EvidenceRef {
            path: "config.yaml".to_string(),
            line: 5,
            parser_error: false,
        }],
    };
    let combined_edge = EvidenceEdge {
        source: source_node,
        derived: config_node,
    };

    assert_eq!(combined_edge.source.origin, "src/agent.py");
    assert_eq!(combined_edge.derived.origin, "config.yaml");
}

#[test]
fn reject_a_derivation_cycle() {
    let node1 = EvidenceNode {
        origin: "a.py".to_string(),
        refs: vec![],
    };
    let node2 = EvidenceNode {
        origin: "b.py".to_string(),
        refs: vec![],
    };
    let mut ir = CapabilityIr::default();
    ir.add_evidence_edge(node1.clone(), node2.clone()).unwrap();
    // Adding the reverse edge should form a cycle and be rejected
    assert!(ir.add_evidence_edge(node2, node1).is_err());
}

#[test]
fn preserve_source_spans_through_serialization() {
    let node = EvidenceNode {
        origin: "test.py".to_string(),
        refs: vec![EvidenceRef {
            path: "test.py".to_string(),
            line: 42,
            parser_error: false,
        }],
    };
    let serialized = serde_json::to_string(&node).unwrap();
    let deserialized: EvidenceNode = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.refs[0].line, 42);
}
