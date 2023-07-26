/// Implements RPC calls for  [`author`](https://paritytech.github.io/substrate/master/sc_rpc/author/struct.Author.html) pallet.
#[async_trait::async_trait]
pub trait AuthorRpc {
    /// Session keys type.
    type SessionKeys;

    /// API for [`rotate_keys`](https://paritytech.github.io/substrate/master/sc_rpc/author/struct.Author.html#method.rotate_keys) call.
    async fn author_rotate_keys(&self) -> anyhow::Result<Self::SessionKeys>;
}
