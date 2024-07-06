use std::ffi::{CStr, CString};
use std::ptr::NonNull;

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
    }
}
