use serde::{Deserialize, Serialize};

use crate::expression::Expression;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct JumpVerdictStatement {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GotoVerdictStatement {
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerdictStatement {
    Accept(Option<bool>),
    Drop(Option<bool>),
    Continue(Option<bool>),
    Return(Option<bool>),
    Jump(JumpVerdictStatement),
    Goto(GotoVerdictStatement),
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
#[serde(untagged)]
pub enum CounterStatement {
    Anonymous(AnonymousCounter),
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
pub enum FwdFamily {
    Ip,
    Ip6,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FwdStatement {
    pub dev: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<FwdFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DupStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev: Option<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NatFlag {
    Random,
    #[serde(rename = "fully-random")]
    FullyRandom,
    Persistent,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SnatStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<NatFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DnatStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<NatFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MasqueradeStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<NatFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RedirectStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<NatFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NatStatement {
    Snat(SnatStatement),
    Dnat(DnatStatement),
    Masquerade(MasqueradeStatement),
    Redirect(RedirectStatement),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RejectType {
    #[serde(rename = "tcp reset")]
    TcpReset,
    Icmpx,
    Icmp,
    Icmpv6,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RejectStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub rejecttype: Option<RejectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<Expression>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetOp {
    Add,
    Update,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SetStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<SetOp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Emerg,
    Alert,
    Crit,
    Err,
    Warn,
    Notice,
    Info,
    Debug,
    Audit,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFlag {
    #[serde(rename = "tcp sequence")]
    TcpSequence,
    #[serde(rename = "tcp options")]
    TcpOptions,
    #[serde(rename = "ip options")]
    IpOptions,
    Skuid,
    Ether,
    All,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LogStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaplen: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queue-threshold")]
    pub queue_threshold: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<LogFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MeterStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stmt: Option<Box<Statement>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueueFlag {
    Bypass,
    Fanout,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct QueueStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<QueueFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct VerdictMapStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CtCountStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum XtTypeName {
    Match,
    Target,
    Watcher,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct XtStatement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub typename: Option<XtTypeName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Statement {
    Match(MatchStatement),
    Counter(CounterStatement),
    Mangle(MangleStatement),
    Quota(QuotaStatement),
    Limit(LimitStatement),
    Fwd(FwdStatement),
    Notrack(Option<bool>),
    Dup(DupStatement),
    Nat(NatStatement),
    Reject(RejectStatement),
    Set(SetStatement),
    Log(LogStatement),
    #[serde(rename = "ct helper")]
    CtHelper(Expression),
    Meter(MeterStatement),
    Queue(QueueStatement),
    #[serde(rename = "vmap")]
    VerdictMap(VerdictMapStatement),
    #[serde(rename = "ct count")]
    CtCount(CtCountStatement),
    #[serde(rename = "ct timeout")]
    CtTimeout(Expression),
    #[serde(rename = "ct expectation")]
    CtExpectation(Expression),
    Xt(XtStatement),
    #[serde(untagged)]
    Verdict(VerdictStatement),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod verdict {
        use super::*;

        #[test]
        fn accept_verdict_serialization() {
            let v =
                serde_json::to_string(&Statement::Verdict(VerdictStatement::Accept(None))).unwrap();
            assert_eq!(v, "{\"accept\":null}");
        }
        #[test]
        fn accept_verdict_deserialization() {
            let raw = "{\"accept\":null}";
            let target = Statement::Verdict(VerdictStatement::Accept(None));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
        #[test]
        fn drop_verdict_serialization() {
            let v =
                serde_json::to_string(&Statement::Verdict(VerdictStatement::Drop(None))).unwrap();
            assert_eq!(v, "{\"drop\":null}");
        }
        #[test]
        fn drop_verdict_deserialization() {
            let raw = "{\"drop\":null}";
            let target = Statement::Verdict(VerdictStatement::Drop(None));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
        #[test]
        fn continue_verdict_serialization() {
            let v = serde_json::to_string(&Statement::Verdict(VerdictStatement::Continue(None)))
                .unwrap();
            assert_eq!(v, "{\"continue\":null}");
        }
        #[test]
        fn continue_verdict_deserialization() {
            let raw = "{\"continue\":null}";
            let target = Statement::Verdict(VerdictStatement::Continue(None));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
        #[test]
        fn return_verdict_serialization() {
            let v =
                serde_json::to_string(&Statement::Verdict(VerdictStatement::Return(None))).unwrap();
            assert_eq!(v, "{\"return\":null}");
        }
        #[test]
        fn return_verdict_deserialization() {
            let raw = "{\"return\":null}";
            let target = Statement::Verdict(VerdictStatement::Return(None));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
        #[test]
        fn jump_verdict_serialization() {
            let v = serde_json::to_string(&Statement::Verdict(VerdictStatement::Jump(
                JumpVerdictStatement {
                    target: "jump_target".to_string(),
                },
            )))
            .unwrap();
            assert_eq!(v, "{\"jump\":{\"target\":\"jump_target\"}}");
        }
        #[test]
        fn jump_verdict_deserialization() {
            let raw = "{\"jump\":{\"target\":\"jump_target\"}}";
            let target = Statement::Verdict(VerdictStatement::Jump(JumpVerdictStatement {
                target: "jump_target".to_string(),
            }));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
        #[test]
        fn goto_verdict_serialization() {
            let v = serde_json::to_string(&Statement::Verdict(VerdictStatement::Goto(
                GotoVerdictStatement {
                    target: "goto_target".to_string(),
                },
            )))
            .unwrap();
            assert_eq!(v, "{\"goto\":{\"target\":\"goto_target\"}}");
        }
        #[test]
        fn goto_verdict_deserialization() {
            let raw = "{\"goto\":{\"target\":\"goto_target\"}}";
            let target = Statement::Verdict(VerdictStatement::Goto(GotoVerdictStatement {
                target: "goto_target".to_string(),
            }));
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(target, parsed);
        }
    }
    mod counter {
        use super::*;

        #[test]
        fn anonymous_counter_statement_serialization() {
            let v = serde_json::to_string(&Statement::Counter(CounterStatement::Anonymous(
                AnonymousCounter {
                    packets: Some(2),
                    bytes: Some(3),
                },
            )))
            .unwrap();
            assert_eq!(v, "{\"counter\":{\"packets\":2,\"bytes\":3}}");
        }
        #[test]
        fn anonymous_counter_statement_deserialization() {
            let target = Statement::Counter(CounterStatement::Anonymous(AnonymousCounter {
                packets: Some(2),
                bytes: Some(3),
            }));
            let raw = "{\"counter\":{\"packets\":2,\"bytes\":3}}";
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(parsed, target);
        }
        #[test]
        fn named_counter_statement_serialization() {
            let v = serde_json::to_string(&Statement::Counter(CounterStatement::Named(
                "mycounter".to_string(),
            )))
            .unwrap();
            assert_eq!(v, "{\"counter\":\"mycounter\"}");
        }
        #[test]
        fn named_counter_statement_deserialization() {
            let target = Statement::Counter(CounterStatement::Named("mycounter".to_string()));
            let raw = "{\"counter\":\"mycounter\"}";
            let parsed = serde_json::from_slice::<Statement>(raw.as_bytes()).unwrap();
            assert_eq!(parsed, target);
        }
    }
    mod quota {
        use super::*;

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
    }
    mod limit {
        use super::*;

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
}
