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
#[test]
fn cycle_or_depth_limit_returns_bounded_uncertainty() {
    let mut ir = CapabilityIr::default();

    // Create a chain of 101 nodes
    let mut current_node = EvidenceNode {
        origin: "node_0".to_string(),
        refs: vec![],
    };

    for i in 1..=101 {
        let next_node = EvidenceNode {
            origin: format!("node_{}", i),
            refs: vec![],
        };
        let result = ir.add_evidence_edge(current_node.clone(), next_node.clone());
        if i > 100 {
            assert!(result.is_err(), "Should reject exceeding depth limit");
            assert_eq!(result.unwrap_err(), "derivation depth limit exceeded");
        } else {
            assert!(result.is_ok());
        }
        current_node = next_node;
    }
}
#[test]
fn derived_evidence_identifies_parent_facts() {
    let source_node = EvidenceNode {
        origin: "source_fact.txt".to_string(),
        refs: vec![],
    };
    let derived_node = EvidenceNode {
        origin: "derived_fact.txt".to_string(),
        refs: vec![],
    };
    let mut ir = CapabilityIr::default();
    ir.add_evidence_edge(source_node.clone(), derived_node.clone())
        .unwrap();

    // Assert that the edge connects derived back to source
    let edge = ir.edges.first().unwrap();
    assert_eq!(edge.derived.origin, "derived_fact.txt");
    assert_eq!(edge.source.origin, "source_fact.txt");
}
