use std::ffi::{CStr, CString};
use std::ptr::NonNull;

#[cfg(feature = "libnftables")]
use nftables_sys::*;

#[cfg(feature = "libnftables")]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NftCtx {
    inner: NonNull<nft_ctx>,
}

#[cfg(feature = "libnftables")]
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

#[cfg(feature = "libnftables")]
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
}
