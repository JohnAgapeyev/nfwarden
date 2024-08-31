use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MapExpression {
    pub key: Box<Expression>,
    pub data: Box<Expression>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PrefixExpression {
    pub addr: Box<Expression>,
    pub len: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PayloadBase {
    #[serde(rename = "ll")]
    LinkLayer,
    #[serde(rename = "nh")]
    NetworkLayer,
    #[serde(rename = "th")]
    TransportLayer,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RawPayload {
    pub base: PayloadBase,
    pub offset: i64,
    pub len: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ReferencePayload {
    pub protocol: String,
    pub field: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PayloadExpression {
    #[serde(untagged)]
    Raw(RawPayload),
    #[serde(untagged)]
    Reference(ReferencePayload),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExtHdrExpression {
    pub name: String,
    pub field: String,
    pub offset: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TcpOptionExpression {
    pub name: String,
    pub field: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SctpChunkExpression {
    pub name: String,
    pub field: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DccpOptionExpression {
    #[serde(rename = "type")]
    pub option_type: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetaKey {
    Length,
    Protocol,
    Priority,
    Random,
    Mark,
    Iif,
    IifName,
    IifType,
    Oif,
    OifName,
    OifType,
    SkUid,
    SkGid,
    NfTrace,
    RtClassId,
    IBriPort,
    OBriPort,
    IBridgeName,
    OBridgeName,
    PktType,
    Cpu,
    IifGroup,
    OifGroup,
    Cgroup,
    NfProto,
    L4Proto,
    SecPath,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MetaExpression {
    pub key: MetaKey,
}

//TODO: Continue implementing
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Expression {
    List(Vec<Box<Expression>>),
    Concat(Vec<Box<Expression>>),
    Set(Vec<Box<Expression>>),
    Map(MapExpression),
    Prefix(PrefixExpression),
    Range([Box<Expression>; 2]),
    Payload(PayloadExpression),
    ExtHdr(ExtHdrExpression),
    #[serde(rename = "tcp option")]
    TcpOption(TcpOptionExpression),
    #[serde(rename = "sctp chunk")]
    SctpChunk(SctpChunkExpression),
    #[serde(rename = "dccp option")]
    DccpOption(DccpOptionExpression),
    Meta(MetaExpression),
    Rt,
    Ct,
    NumGen,
    JHash,
    SymHash,
    Fib,
    //TODO: Implement "Binary Operation" Expression
    BinOp,
    Verdict,
    Elem,
    Socket,
    Osf,
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    Number(i64),
    #[serde(untagged)]
    Boolean(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod concat {
        use super::*;

        #[test]
        fn concat_serialization() {
            let v = serde_json::to_string(&Expression::Concat(vec![
                Box::new(Expression::String("a".to_string())),
                Box::new(Expression::String("b".to_string())),
                Box::new(Expression::String("c".to_string())),
            ]))
            .unwrap();
            assert_eq!(v, "{\"concat\":[\"a\",\"b\",\"c\"]}");
        }
    }
    mod set {
        use super::*;

        #[test]
        fn set_serialization() {
            let v = serde_json::to_string(&Expression::Set(vec![
                Box::new(Expression::String("a".to_string())),
                Box::new(Expression::String("b".to_string())),
                Box::new(Expression::String("c".to_string())),
            ]))
            .unwrap();
            assert_eq!(v, "{\"set\":[\"a\",\"b\",\"c\"]}");
        }
    }
    mod map {
        use super::*;

        #[test]
        fn map_serialization() {
            let v = serde_json::to_string(&Expression::Map(MapExpression {
                key: Box::new(Expression::String("a".to_string())),
                data: Box::new(Expression::String("b".to_string())),
            }))
            .unwrap();
            assert_eq!(v, "{\"map\":{\"key\":\"a\",\"data\":\"b\"}}");
        }
    }
    mod prefix {
        use super::*;

        #[test]
        fn addr_prefix_serialization() {
            let v = serde_json::to_string(&Expression::Prefix(PrefixExpression {
                addr: Box::new(Expression::String("127.0.0.1".to_string())),
                len: 3,
            }))
            .unwrap();
            assert_eq!(v, "{\"prefix\":{\"addr\":\"127.0.0.1\",\"len\":3}}");
        }
    }
    mod range {
        use super::*;

        #[test]
        fn range_serialization() {
            let v = serde_json::to_string(&Expression::Range([
                Box::new(Expression::Number(2)),
                Box::new(Expression::Number(4)),
            ]))
            .unwrap();
            assert_eq!(v, "{\"range\":[2,4]}");
        }
    }
    mod payload {
        use super::*;

        #[test]
        fn raw_payload_serialization() {
            let v =
                serde_json::to_string(&Expression::Payload(PayloadExpression::Raw(RawPayload {
                    base: PayloadBase::LinkLayer,
                    offset: 13,
                    len: 27,
                })))
                .unwrap();
            assert_eq!(
                v,
                "{\"payload\":{\"base\":\"ll\",\"offset\":13,\"len\":27}}"
            );
        }
        #[test]
        fn reference_payload_serialization() {
            let v = serde_json::to_string(&Expression::Payload(PayloadExpression::Reference(
                ReferencePayload {
                    protocol: "tcp".to_string(),
                    field: "port".to_string(),
                },
            )))
            .unwrap();
            assert_eq!(v, "{\"payload\":{\"protocol\":\"tcp\",\"field\":\"port\"}}");
        }
    }
    mod exthdr {
        use super::*;

        #[test]
        fn exthdr_serialization() {
            let v = serde_json::to_string(&Expression::ExtHdr(ExtHdrExpression {
                name: "a".to_string(),
                field: "b".to_string(),
                offset: 3,
            }))
            .unwrap();
            assert_eq!(
                v,
                "{\"exthdr\":{\"name\":\"a\",\"field\":\"b\",\"offset\":3}}"
            );
        }
    }
    mod tcp_option {
        use super::*;

        #[test]
        fn tcp_option_serialization() {
            let v = serde_json::to_string(&Expression::TcpOption(TcpOptionExpression {
                name: "a".to_string(),
                field: "b".to_string(),
            }))
            .unwrap();
            assert_eq!(v, "{\"tcp option\":{\"name\":\"a\",\"field\":\"b\"}}");
        }
    }
    mod sctp_chunk {
        use super::*;

        #[test]
        fn sctp_chunk_serialization() {
            let v = serde_json::to_string(&Expression::SctpChunk(SctpChunkExpression {
                name: "a".to_string(),
                field: "b".to_string(),
            }))
            .unwrap();
            assert_eq!(v, "{\"sctp chunk\":{\"name\":\"a\",\"field\":\"b\"}}");
        }
    }
    mod dccp_option {
        use super::*;

        #[test]
        fn dccp_option_serialization() {
            let v = serde_json::to_string(&Expression::DccpOption(DccpOptionExpression {
                option_type: 1,
            }))
            .unwrap();
            assert_eq!(v, "{\"dccp option\":{\"type\":1}}");
        }
    }
}
