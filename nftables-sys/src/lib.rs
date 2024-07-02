#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctx_alloc_free() {
        let first_ctx = unsafe { nft_ctx_new(NFT_CTX_DEFAULT) };
        assert!(!first_ctx.is_null());

        let second_ctx = unsafe { nft_ctx_new(NFT_CTX_DEFAULT) };
        assert!(!second_ctx.is_null());

        assert_ne!(first_ctx, second_ctx);

        unsafe { nft_ctx_free(second_ctx) };
        unsafe { nft_ctx_free(first_ctx) };
    }

    #[test]
    fn dry_run_flag() {
        let ctx = unsafe { nft_ctx_new(NFT_CTX_DEFAULT) };
        assert!(!ctx.is_null());

        let old = unsafe { nft_ctx_get_dry_run(ctx) };
        assert_eq!(old, false);

        unsafe { nft_ctx_set_dry_run(ctx, true) };
        let new = unsafe { nft_ctx_get_dry_run(ctx) };
        assert_eq!(new, true);
        assert_ne!(old, new);

        unsafe { nft_ctx_free(ctx) };
    }
}
