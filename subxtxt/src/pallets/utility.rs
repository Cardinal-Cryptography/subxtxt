use crate::{connection::TxInfo, TxStatus};

/// Pallet utility api.
#[async_trait::async_trait]
pub trait UtilityApi {
    /// Runtime call API.
    type Call;

    /// API for [`batch`](https://paritytech.github.io/substrate/master/pallet_utility/pallet/struct.Pallet.html#method.batch) call.
    async fn batch_call(&self, calls: Vec<Self::Call>, status: TxStatus) -> anyhow::Result<TxInfo>;
}
