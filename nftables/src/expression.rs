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
    Range,
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
