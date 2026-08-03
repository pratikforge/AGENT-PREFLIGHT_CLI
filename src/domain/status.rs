use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Verified,
    Failed,
    Partial,
    CannotVerifyStatically,
    Unsupported,
}

impl Status {
    pub const fn exit_code(self) -> i32 {
        match self {
            Self::Verified => 0,
            Self::Failed => 10,
            Self::Partial => 11,
            Self::CannotVerifyStatically => 12,
            Self::Unsupported => 13,
        }
    }
}
