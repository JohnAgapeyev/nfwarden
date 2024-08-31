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
    Payload,
    ExtHdr,
    #[serde(rename = "tcp option")]
    TcpOption,
    #[serde(rename = "sctp chunk")]
    SctpChunk,
    #[serde(rename = "dccp option")]
    DccpOption,
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
}
