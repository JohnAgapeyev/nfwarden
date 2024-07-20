use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use serde::{Deserialize, Serialize};

use nftables_sys::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    Something,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NftCtx {
    inner: NonNull<nft_ctx>,
}

impl NftCtx {
    pub fn new() -> Self {
        let inner = unsafe {
            let ctx =
                NonNull::new(nft_ctx_new(NFT_CTX_DEFAULT)).expect("Could not allocate nft_ctx");
            if nft_ctx_buffer_output(ctx.as_ptr()) != 0 {
                nft_ctx_free(ctx.as_ptr());
                panic!("Creating the output buffer failed!");
            }
            if nft_ctx_buffer_error(ctx.as_ptr()) != 0 {
                nft_ctx_free(ctx.as_ptr());
                panic!("Creating the error buffer failed!");
            }
            ctx
        };
        Self { inner }
    }

    pub fn is_dry_run(&self) -> bool {
        unsafe { nft_ctx_get_dry_run(self.inner.as_ptr()) }
    }
    pub fn set_dry_run(&mut self, dry_run: bool) {
        unsafe { nft_ctx_set_dry_run(self.inner.as_ptr(), dry_run) }
    }
    pub fn set_json(&mut self) {
        unsafe { nft_ctx_input_set_flags(self.inner.as_ptr(), NFT_CTX_INPUT_JSON) };
        unsafe { nft_ctx_output_set_flags(self.inner.as_ptr(), NFT_CTX_OUTPUT_JSON) };
    }

    pub fn run_cmd_bytes(&mut self, cmd: &CStr) -> Result<String, String> {
        let ret = unsafe { nft_run_cmd_from_buffer(self.inner.as_ptr(), cmd.as_ptr()) };
        if ret == 0 {
            let output = unsafe { CStr::from_ptr(nft_ctx_get_output_buffer(self.inner.as_ptr())) }
                .to_str()
                .expect("Library output is not valid UTF-8")
                .to_string();
            return Ok(output);
        }
        let error = unsafe { CStr::from_ptr(nft_ctx_get_error_buffer(self.inner.as_ptr())) }
            .to_str()
            .expect("Library error is not valid UTF-8")
            .to_string();
        return Err(error);
    }
    pub fn run_cmd_str(&mut self, cmd: &str) -> Result<String, String> {
        let converted = CString::new(cmd).unwrap();
        self.run_cmd_bytes(&converted)
    }
}

impl Drop for NftCtx {
    fn drop(&mut self) {
        unsafe { nft_ctx_free(self.inner.as_ptr()) };
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NftMeta {
    pub version: String,
    pub release_name: String,
    pub json_schema_version: u64,
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
#[serde(rename_all = "lowercase")]
pub enum NftObjects {
    #[serde(rename = "metainfo")]
    Meta(NftMeta),
    Table(NftTableList),
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
    fn dry_run_toggling() {
        let mut ctx = NftCtx::new();
        assert_eq!(ctx.is_dry_run(), false);
        ctx.set_dry_run(true);
        assert_eq!(ctx.is_dry_run(), true);
        ctx.set_dry_run(false);
        assert_eq!(ctx.is_dry_run(), false);
    }

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
            println!("JSON Parsed:\n{parsed:#?}");
        }

        //let input = NftOutput {
        //    items: vec![NftObjects::Table(NftTableList {
        //        family: None,
        //        name: None,
        //        handle: None,
        //    })],
        //};
        //let test = ctx.run_cmd_str(&serde_json::to_string(&input).unwrap());

        //dbg!(test);

        ////println!("{}", serde_json::to_string(&output).unwrap());
        //if let Ok(raw) = ctx.run_cmd_str(&serde_json::to_string(&input).unwrap()) {
        //    let parsed = serde_json::from_slice::<NftOutput>(raw.as_bytes()).unwrap();
        //    println!("FANCY JSON Parsed:\n{parsed:#?}");
        //}

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
