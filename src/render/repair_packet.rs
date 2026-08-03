use crate::domain::evidence::EvidenceRef;

pub fn render(rule_id: &str, evidence: &EvidenceRef, revision: &str) -> String {
    format!(
        "# Repair packet: `{}`\n\n## Evidence\n\n- File: `{}`\n- Line: {}\n- Contract revision: `{}`\n\n## Missing rule\n\nThe approved structural rule is currently failed.\n\n## Allowed change boundary\n\nChange only the documented framework control at the cited source location.\n\n## Non-goals\n\n- Do not execute the agent or its tools.\n- Do not change unrelated source, dependencies, or CI.\n- Do not treat this packet as an automatic patch.\n\n## Acceptance checks\n\n- The cited rule is structurally verified after a manual repair.\n- The contract revision remains current.\n\n## Exact verify command\n\n```text\nagent-preflight verify . --ci\n```\n",
        escape_inline(rule_id),
        escape_inline(&evidence.path),
        evidence.line,
        escape_inline(revision),
    )
}

fn escape_inline(value: &str) -> String {
    value.replace('`', "'").replace(['\r', '\n'], " ")
}
