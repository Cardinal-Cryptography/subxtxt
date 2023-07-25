use subxt::ext::sp_runtime::FixedU128;

use crate::BlockHash;

/// Transaction payment pallet API.
#[async_trait::async_trait]
pub trait TransactionPaymentApi {
    /// API for [`next_fee_multiplier`](https://paritytech.github.io/substrate/master/pallet_transaction_payment/pallet/struct.Pallet.html#method.next_fee_multiplier) call.
    async fn get_next_fee_multiplier(&self, at: Option<BlockHash>) -> FixedU128;
}
