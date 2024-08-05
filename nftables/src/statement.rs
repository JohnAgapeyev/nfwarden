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
pub enum CounterStatement {
    #[serde(rename = "counter")]
    Anonymous(AnonymousCounter),
    #[serde(rename = "counter")]
    Named(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MangleStatement {
    //TODO: Do we need to limit things to "exthdr, payload, meta, ct, or ct helper" expression?
    pub key: Expression,
    pub value: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AnonymousQuota {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum QuotaStatement {
    #[serde(rename = "quota")]
    Anonymous(AnonymousQuota),
    #[serde(rename = "quota")]
    Named(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AnonymousLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum LimitStatement {
    #[serde(rename = "limit")]
    Anonymous(AnonymousLimit),
    #[serde(rename = "limit")]
    Named(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Statement {
    Verdict(VerdictStatement),
    Match(MatchStatement),
    Counter(CounterStatement),
    Mangle(MangleStatement),
    Quota(QuotaStatement),
    Limit(LimitStatement),
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
    #[test]
    fn anonymous_quota_statement_serialization() {
        let v = serde_json::to_string(&QuotaStatement::Anonymous(AnonymousQuota {
            val: Some(1),
            val_unit: Some("kbytes".to_string()),
            used: Some(0),
            used_unit: Some("kbytes".to_string()),
            inv: Some(false),
        }))
        .unwrap();
        assert_eq!(v, "{\"quota\":{\"val\":1,\"val_unit\":\"kbytes\",\"used\":0,\"used_unit\":\"kbytes\",\"inv\":false}}");
    }
    #[test]
    fn named_quota_statement_serialization() {
        let v = serde_json::to_string(&QuotaStatement::Named("myquota".to_string())).unwrap();
        assert_eq!(v, "{\"quota\":\"myquota\"}");
    }
    #[test]
    fn anonymous_limit_statement_serialization() {
        let v = serde_json::to_string(&LimitStatement::Anonymous(AnonymousLimit {
            rate: Some(1),
            rate_unit: Some("kbytes".to_string()),
            per: Some("minutes".to_string()),
            burst: Some(0),
            burst_unit: Some("kbytes".to_string()),
            inv: Some(false),
        }))
        .unwrap();
        assert_eq!(v, "{\"limit\":{\"rate\":1,\"rate_unit\":\"kbytes\",\"per\":\"minutes\",\"burst\":0,\"burst_unit\":\"kbytes\",\"inv\":false}}");
    }
    #[test]
    fn named_limit_statement_serialization() {
        let v = serde_json::to_string(&LimitStatement::Named("mylimit".to_string())).unwrap();
        assert_eq!(v, "{\"limit\":\"mylimit\"}");
    }
}
