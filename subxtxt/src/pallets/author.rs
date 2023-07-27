/// Implements RPC calls for  [`author`](https://paritytech.github.io/substrate/master/sc_rpc/author/struct.Author.html) pallet.
#[async_trait::async_trait]
pub trait AuthorRpc {
    /// Session keys type.
    type SessionKeys;

    /// API for [`rotate_keys`](https://paritytech.github.io/substrate/master/sc_rpc/author/struct.Author.html#method.rotate_keys) call.
    async fn author_rotate_keys(&self) -> anyhow::Result<Self::SessionKeys>;
}

macro_rules! pallet_api_impl {
    ($SessionKeys: ty) => {
        #[async_trait::async_trait]
        impl<C: ::subxtxt::connection::AsConnection + Sync> AuthorRpc for C {
            type SessionKeys = $SessionKeys;

            async fn author_rotate_keys(&self) -> anyhow::Result<$SessionKeys> {
                use parity_scale_codec::Decode;

                let bytes = self.as_connection().as_client().rpc().rotate_keys().await?;
                <$SessionKeys>::decode(&mut bytes.0.as_slice()).map_err(|e| e.into())
            }
        }
    };
}
