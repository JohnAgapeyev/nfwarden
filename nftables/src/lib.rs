use std::ptr::NonNull;

use nftables_sys::*;

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
}
