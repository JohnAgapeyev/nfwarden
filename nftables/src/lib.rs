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
    fn it_works() {
        assert_eq!(true, true);
    }
}
