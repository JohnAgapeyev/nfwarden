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
        Self {
            inner: NonNull::new(unsafe { nft_ctx_new(NFT_CTX_DEFAULT) })
                .expect("Could not allocate nft_ctx"),
        }
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

    pub fn run_cmd_bytes(&mut self, cmd: &CStr) -> Result<(), Error> {
        let ret = unsafe { nft_run_cmd_from_buffer(self.inner.as_ptr(), cmd.as_ptr()) };
        if ret == 0 {
            return Ok(());
        }
        //TODO: Convert library error into a proper return value
        unimplemented!();
    }
    pub fn run_cmd_str(&mut self, cmd: &str) -> Result<(), Error> {
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
    pub family: String,
    pub name: String,
    pub handle: u64,
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
        ctx.run_cmd_str(
            "{ \"nftables\": [ { \"list\": { \"tables\": { \"family\": \"ip\" } } } ] }",
        )
        .unwrap();

        let parsed = serde_json::from_slice::<NftOutput>("{\"nftables\": [{\"metainfo\": {\"version\": \"1.0.9\", \"release_name\": \"Old Doc Yak #3\", \"json_schema_version\": 1}}, {\"table\": {\"family\": \"ip\", \"name\": \"libvirt_network\", \"handle\": 1}}]}".as_bytes()).unwrap();
        println!("{parsed:#?}");

        let output = NftOutput {
            items: vec![
                NftObjects::Meta(NftMeta {
                    version: "1.0.9".to_string(),
                    release_name: "Old Doc Yak #3".to_string(),
                    json_schema_version: 1,
                }),
                NftObjects::Table(NftTableList {
                    family: "ip".to_string(),
                    name: "libvirt_network".to_string(),
                    handle: 1,
                }),
            ],
        };
        println!("{}", serde_json::to_string(&output).unwrap());
    }
}
