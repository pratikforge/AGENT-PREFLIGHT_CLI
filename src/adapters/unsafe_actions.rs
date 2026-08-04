use crate::domain::evidence::EvidenceRef;
use crate::domain::normalized::NormalizedFile;
use crate::domain::status::Status;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Finding {
    pub rule_id: String,
    pub status: Status,
    pub evidence: EvidenceRef,
    pub matrix_source: String,
}

pub fn evaluate(files: &[NormalizedFile]) -> Vec<Finding> {
    let mut findings = Vec::new();

    for file in files {
        for call in &file.calls {
            // raw eval interception
            if call.callee == "eval" || call.callee.contains("exec") {
                findings.push(Finding {
                    rule_id: "unsafe-eval".to_string(),
                    status: Status::Failed,
                    evidence: EvidenceRef {
                        path: file.path.clone(),
                        line: call.span.line,
                        parser_error: false,
                    },
                    matrix_source: "UNSAFE".to_string(),
                });
            }

            // run_command analysis
            if call.callee.contains("run_command") || call.callee.contains("subprocess") {
                let mut is_unsafe = false;
                for ctrl in &call.static_controls {
                    if ctrl.contains("rm -rf") || ctrl.contains("format ") || ctrl.contains("mkfs")
                    {
                        is_unsafe = true;
                        findings.push(Finding {
                            rule_id: "unsafe-rm-rf".to_string(),
                            status: Status::Failed,
                            evidence: EvidenceRef {
                                path: file.path.clone(),
                                line: call.span.line,
                                parser_error: false,
                            },
                            matrix_source: "UNSAFE".to_string(),
                        });
                    }
                }
                if !is_unsafe {
                    findings.push(Finding {
                        rule_id: "safe-command".to_string(),
                        status: Status::Verified,
                        evidence: EvidenceRef {
                            path: file.path.clone(),
                            line: call.span.line,
                            parser_error: false,
                        },
                        matrix_source: "UNSAFE".to_string(),
                    });
                }
            }
        }
    }
    findings
}
