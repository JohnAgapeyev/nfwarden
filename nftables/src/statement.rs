use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct JumpVerdict {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GotoVerdict {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerdictStatement {
    Accept(Option<bool>),
    Drop(Option<bool>),
    Continue(Option<bool>),
    Return(Option<bool>),
    Jump(JumpVerdict),
    Goto(GotoVerdict),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Statement {
    Verdict,
    Match,
    Counter,
    Mangle,
    Quota,
    Limit,
    Fwd,
    Notrack,
    Dup,
    Nat,
    Reject,
    Set,
    Log,
    CtHelper,
    Meter,
    Queue,
    VerdictMap,
    CtCount,
    CtTimeout,
    CtExpectation,
    Xt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Accept(None)).unwrap();
        assert_eq!(v, "{\"accept\":null}");
    }
    #[test]
    fn drop_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Drop(None)).unwrap();
        assert_eq!(v, "{\"drop\":null}");
    }
    #[test]
    fn continue_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Continue(None)).unwrap();
        assert_eq!(v, "{\"continue\":null}");
    }
    #[test]
    fn return_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Return(None)).unwrap();
        assert_eq!(v, "{\"return\":null}");
    }
    #[test]
    fn jump_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Jump(JumpVerdict {
            target: "jump_target".to_string(),
        }))
        .unwrap();
        assert_eq!(v, "{\"jump\":{\"target\":\"jump_target\"}}");
    }
    #[test]
    fn goto_verdict_serialization() {
        let v = serde_json::to_string(&VerdictStatement::Goto(GotoVerdict {
            target: "goto_target".to_string(),
        }))
        .unwrap();
        assert_eq!(v, "{\"goto\":{\"target\":\"goto_target\"}}");
    }
}
