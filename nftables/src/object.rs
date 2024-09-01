use serde::{Deserialize, Serialize};

use crate::expression::*;
use crate::statement::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TableFlag {
    Dormant,
    Owner,
    Persist,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TableElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<TableFlag>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ChainElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub chaintype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prio: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RuleElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<Vec<Box<Statement>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetPolicy {
    Performance,
    Memory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SetFlag {
    Constant,
    Interval,
    Timeout,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Set {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub settype: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<SetPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<SetFlag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<Box<Expression>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gc-interval")]
    pub gc_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "auto-merge")]
    pub auto_merge: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Map {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub settype: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<SetPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<SetFlag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<Box<Expression>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gc-interval")]
    pub gc_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "auto-merge")]
    pub auto_merge: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Element {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<Box<Expression>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FlowtableElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<Box<Expression>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CounterElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct QuotaElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CtHelperProto {
    Tcp,
    Udp,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CtHelperElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub cttype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<CtHelperProto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3proto: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LimitUnit {
    Packets,
    Bytes,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LimitElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<LimitUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MetaInfoElement {
    pub version: String,
    pub release_name: String,
    pub json_schema_version: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CtTimeoutProto {
    Tcp,
    Udp,
    Dccp,
    Sctp,
    Gre,
    Icmpv6,
    Icmp,
    Generic,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CtTimeoutElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<CtTimeoutProto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3proto: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CtExpectationProto {
    Tcp,
    Udp,
    Dccp,
    Sctp,
    Gre,
    Icmpv6,
    Icmp,
    Generic,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CtExpectationElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<CtExpectationProto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dport: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Object {
    Table(TableElement),
    Chain(ChainElement),
    Rule(RuleElement),
    Set(Set),
    Map(Map),
    Element(Element),
    Flowtable(FlowtableElement),
    Counter(CounterElement),
    Quota(QuotaElement),
    ConnectionTrackHelper(CtHelperElement),
    Limit(LimitElement),
    MetaInfo(MetaInfoElement),
    ConnectionTrackTimeout(CtTimeoutElement),
    ConnectionTrackExpectation(CtExpectationElement),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommandObject {
    Add(Object),
    Replace(Object),
    Create(Object),
    Insert(Object),
    Delete(Object),
    List(Object),
    Reset(Object),
    Flush(Object),
    Rename(Object),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Command {
    //TODO: This works but is super ugly, is there a better way?
    #[serde(rename = "nftables")]
    pub objects: Vec<CommandObject>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CommandResponse {
    //TODO: This works but is super ugly, is there a better way?
    #[serde(rename = "nftables")]
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftTableListOutput {
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftTableList {
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftList {
    pub tables: NftTableList,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftMeta {
    pub version: String,
    pub release_name: String,
    pub json_schema_version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NftObjects {
    #[serde(rename = "metainfo")]
    Meta(NftMeta),
    Table(NftTableListOutput),
    List(NftList),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftOutput {
    #[serde(rename = "nftables")]
    pub items: Vec<NftObjects>,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod rule {
        use super::*;

        #[test]
        fn rule_serialize_deserialize() {
            let raw="{\"rule\":{\"family\":\"ip\",\"table\":\"libvirt_network\",\"chain\":\"forward\",\"handle\":7,\"expr\":[{\"counter\":{\"packets\":0,\"bytes\":0}},{\"jump\":{\"target\":\"guest_cross\"}}]}}".to_string();

            let v = Object::Rule(RuleElement {
                family: Some("ip".to_string()),
                table: Some("libvirt_network".to_string()),
                chain: Some("forward".to_string()),
                expr: Some(vec![
                    Box::new(Statement::Counter(CounterStatement::Anonymous(
                        AnonymousCounter {
                            packets: Some(0),
                            bytes: Some(0),
                        },
                    ))),
                    Box::new(Statement::Verdict(VerdictStatement::Jump(
                        JumpVerdictStatement {
                            target: "guest_cross".to_string(),
                        },
                    ))),
                ]),
                handle: Some(7),
                index: None,
                comment: None,
            });

            let parsed = serde_json::from_slice::<Object>(raw.as_bytes()).unwrap();

            assert_eq!(v, parsed);
        }
    }
}
