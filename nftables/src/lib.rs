use serde::{Deserialize, Serialize};

pub mod ctx;
pub use ctx::*;

pub mod object;
pub use object::*;

pub mod statement;
pub use statement::*;

pub mod expression;
pub use expression::*;

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

    #[test]
    fn basic_command_running() {
        let mut ctx = NftCtx::new();
        ctx.set_json();
        ctx.set_dry_run(true);
        let res = ctx.run_cmd_str(
            //"{ \"nftables\": [ { \"list\": { \"tables\": { \"family\": \"ip\" } } } ] }",
            "{ \"nftables\": [ { \"list\": { \"tables\": { \"family\": null } } } ] }",
        );

        match res {
            Ok(s) => println!("MY LIBRARY OUTPUT: {s}"),
            Err(s) => println!("MY LIBRARY ERROR: {s}"),
        }

        if let Ok(raw) = ctx
            .run_cmd_str("{ \"nftables\": [ { \"list\": { \"tables\": { \"family\": null } } } ] }")
        {
            let parsed = serde_json::from_slice::<NftOutput>(raw.as_bytes()).unwrap();
            println!("Raw JSON Parsed:\n{parsed:#?}");
        }

        let input = NftOutput {
            items: vec![NftObjects::List(NftList {
                tables: NftTableList {
                    family: None,
                    name: None,
                    handle: None,
                },
            })],
        };

        println!("{}", serde_json::to_string(&input).unwrap());
        if let Ok(raw) = ctx.run_cmd_str(&serde_json::to_string(&input).unwrap()) {
            let parsed = serde_json::from_slice::<NftOutput>(raw.as_bytes()).unwrap();
            println!("Serialized Input JSON Parsed:\n{parsed:#?}");
        }

        //let parsed = serde_json::from_slice::<NftOutput>("{\"nftables\": [{\"metainfo\": {\"version\": \"1.0.9\", \"release_name\": \"Old Doc Yak #3\", \"json_schema_version\": 1}}, {\"table\": {\"family\": \"ip\", \"name\": \"libvirt_network\", \"handle\": 1}}]}".as_bytes()).unwrap();
        //println!("{parsed:#?}");

        //let output = NftOutput {
        //    items: vec![
        //        NftObjects::Meta(NftMeta {
        //            version: "1.0.9".to_string(),
        //            release_name: "Old Doc Yak #3".to_string(),
        //            json_schema_version: 1,
        //        }),
        //        NftObjects::Table(NftTableList {
        //            family: "ip".to_string(),
        //            name: "libvirt_network".to_string(),
        //            handle: 1,
        //        }),
        //    ],
        //};
        //println!("{}", serde_json::to_string(&output).unwrap());
    }

    #[test]
    fn complex_chain_response_parsing() {
        let mut ctx = NftCtx::new();
        ctx.set_json();
        ctx.set_dry_run(true);

        let raw: String = "
{\"nftables\": [{\"metainfo\": {\"version\": \"1.1.0\", \"release_name\": \"Commodore Bullmoose\", \"json_schema_version\": 1}}, {\"table\": {\"family\": \"ip\", \"name\": \"libvirt_network\", \"handle\": 1}}, {\"chain\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"name\": \"forward\", \"handle\": 1, \"type\": \"filter\", \"hook\": \"forward\", \"prio\": 0, \"policy\": \"accept\"}}, {\"chain\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"name\": \"guest_output\", \"handle\": 2}}, {\"chain\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"name\": \"guest_input\", \"handle\": 4}}, {\"chain\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"name\": \"guest_cross\", \"handle\": 6}}, {\"chain\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"name\": \"guest_nat\", \"handle\": 8, \"type\": \"nat\", \"hook\": \"postrouting\", \"prio\": 100, \"policy\": \"accept\"}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"forward\", \"handle\": 7, \"expr\": [{\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"jump\": {\"target\": \"guest_cross\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"forward\", \"handle\": 5, \"expr\": [{\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"jump\": {\"target\": \"guest_input\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"forward\", \"handle\": 3, \"expr\": [{\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"jump\": {\"target\": \"guest_output\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_output\", \"handle\": 24, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iif\"}}, \"right\": \"virbr1\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"reject\": {\"type\": \"icmp\", \"expr\": \"port-unreachable\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_output\", \"handle\": 13, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iif\"}}, \"right\": \"virbr0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"accept\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_output\", \"handle\": 10, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iif\"}}, \"right\": \"virbr0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"reject\": {\"type\": \"icmp\", \"expr\": \"port-unreachable\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_input\", \"handle\": 25, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"oif\"}}, \"right\": \"virbr1\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"reject\": {\"type\": \"icmp\", \"expr\": \"port-unreachable\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_input\", \"handle\": 16, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"oif\"}}, \"right\": \"virbr0\"}}, {\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"in\", \"left\": {\"ct\": {\"key\": \"state\"}}, \"right\": [\"established\", \"related\"]}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"accept\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_input\", \"handle\": 11, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"oif\"}}, \"right\": \"virbr0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"reject\": {\"type\": \"icmp\", \"expr\": \"port-unreachable\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_cross\", \"handle\": 26, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iif\"}}, \"right\": \"virbr1\"}}, {\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"oif\"}}, \"right\": \"virbr1\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"accept\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_cross\", \"handle\": 12, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iif\"}}, \"right\": \"virbr0\"}}, {\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"oif\"}}, \"right\": \"virbr0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"accept\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_nat\", \"handle\": 23, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"224.0.0.0\", \"len\": 24}}}}, {\"counter\": {\"packets\": 333, \"bytes\": 24276}}, {\"return\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_nat\", \"handle\": 22, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": \"255.255.255.255\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"return\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_nat\", \"handle\": 21, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"l4proto\"}}, \"right\": \"tcp\"}}, {\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"!=\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"masquerade\": {\"port\": {\"range\": [1024, 65535]}}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_nat\", \"handle\": 20, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"l4proto\"}}, \"right\": \"udp\"}}, {\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"!=\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"counter\": {\"packets\": 332, \"bytes\": 50796}}, {\"masquerade\": {\"port\": {\"range\": [1024, 65535]}}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"libvirt_network\", \"chain\": \"guest_nat\", \"handle\": 19, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"match\": {\"op\": \"!=\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"192.168.122.0\", \"len\": 24}}}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"masquerade\": null}]}}]}
            ".to_string();

        let parsed = serde_json::from_slice::<CommandResponse>(raw.as_bytes()).unwrap();
        println!("Serialized Input JSON Parsed:\n{parsed:#?}");
    }

    #[test]
    fn docker_nat_table_parsing() {
        let mut ctx = NftCtx::new();
        ctx.set_json();
        ctx.set_dry_run(true);

        let raw: String = "
        {\"nftables\": [{\"metainfo\": {\"version\": \"1.1.0\", \"release_name\": \"Commodore Bullmoose\", \"json_schema_version\": 1}}, {\"table\": {\"family\": \"ip\", \"name\": \"nat\", \"handle\": 3}}, {\"chain\": {\"family\": \"ip\", \"table\": \"nat\", \"name\": \"DOCKER\", \"handle\": 1}}, {\"chain\": {\"family\": \"ip\", \"table\": \"nat\", \"name\": \"POSTROUTING\", \"handle\": 2, \"type\": \"nat\", \"hook\": \"postrouting\", \"prio\": 100, \"policy\": \"accept\"}}, {\"chain\": {\"family\": \"ip\", \"table\": \"nat\", \"name\": \"PREROUTING\", \"handle\": 5, \"type\": \"nat\", \"hook\": \"prerouting\", \"prio\": -100, \"policy\": \"accept\"}}, {\"chain\": {\"family\": \"ip\", \"table\": \"nat\", \"name\": \"OUTPUT\", \"handle\": 7, \"type\": \"nat\", \"hook\": \"output\", \"prio\": -100, \"policy\": \"accept\"}}, {\"rule\": {\"family\": \"ip\", \"table\": \"nat\", \"chain\": \"DOCKER\", \"handle\": 10, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"meta\": {\"key\": \"iifname\"}}, \"right\": \"docker0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"return\": null}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"nat\", \"chain\": \"POSTROUTING\", \"handle\": 9, \"expr\": [{\"match\": {\"op\": \"==\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"saddr\"}}, \"right\": {\"prefix\": {\"addr\": \"172.17.0.0\", \"len\": 16}}}}, {\"match\": {\"op\": \"!=\", \"left\": {\"meta\": {\"key\": \"oifname\"}}, \"right\": \"docker0\"}}, {\"counter\": {\"packets\": 0, \"bytes\": 0}}, {\"xt\": {\"type\": \"target\", \"name\": \"MASQUERADE\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"nat\", \"chain\": \"PREROUTING\", \"handle\": 6, \"expr\": [{\"xt\": {\"type\": \"match\", \"name\": \"addrtype\"}}, {\"counter\": {\"packets\": 52, \"bytes\": 3471}}, {\"jump\": {\"target\": \"DOCKER\"}}]}}, {\"rule\": {\"family\": \"ip\", \"table\": \"nat\", \"chain\": \"OUTPUT\", \"handle\": 8, \"expr\": [{\"match\": {\"op\": \"!=\", \"left\": {\"payload\": {\"protocol\": \"ip\", \"field\": \"daddr\"}}, \"right\": {\"prefix\": {\"addr\": \"127.0.0.0\", \"len\": 8}}}}, {\"xt\": {\"type\": \"match\", \"name\": \"addrtype\"}}, {\"counter\": {\"packets\": 4, \"bytes\": 240}}, {\"jump\": {\"target\": \"DOCKER\"}}]}}]}
            ".to_string();

        let parsed = serde_json::from_slice::<CommandResponse>(raw.as_bytes()).unwrap();
        println!("Serialized Input JSON Parsed:\n{parsed:#?}");
    }
}
