use serde::{Deserialize, Serialize};

use crate::expression::Expression;

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
pub enum MatchOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<=")]
    LessThanEqual,
    #[serde(rename = ">=")]
    GreaterThanEqual,
    #[serde(rename = "in")]
    In,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MatchStatement {
    pub left: Expression,
    pub right: Expression,
    pub op: MatchOp,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AnonymousCounter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NamedCounter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CounterStatement {
    #[serde(rename = "counter")]
    Anonymous(AnonymousCounter),
    #[serde(rename = "counter")]
    Named(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Statement {
    Verdict(VerdictStatement),
    Match(MatchStatement),
    Counter(CounterStatement),
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
    #[test]
    fn anonymous_counter_statement_serialization() {
        let v = serde_json::to_string(&CounterStatement::Anonymous(AnonymousCounter {
            packets: Some(2),
            bytes: Some(3),
        }))
        .unwrap();
        assert_eq!(v, "{\"counter\":{\"packets\":2,\"bytes\":3}}");
    }
    #[test]
    fn named_counter_statement_serialization() {
        let v = serde_json::to_string(&CounterStatement::Named("mycounter".to_string())).unwrap();
        assert_eq!(v, "{\"counter\":\"mycounter\"}");
    }
}
