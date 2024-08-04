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
}
