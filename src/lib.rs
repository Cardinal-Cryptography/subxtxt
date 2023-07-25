//! Convenient extensions to the `subxt` library.

#![warn(missing_docs)]

use subxt::ext::sp_core::{crypto::AccountId32, sr25519, H256};

pub mod connection;
mod key_pair;
pub mod pallets;

pub use key_pair::*;

/// An alias for a type of a key pair that signs chain transactions.
pub type RawKeyPair = sr25519::Pair;
/// An alias for an account id type.
pub type AccountId = AccountId32;
/// An alias for a hash type.
pub type CodeHash = H256;
/// An alias for a block hash type.
pub type BlockHash = H256;
/// An alias for a block number type.
pub type BlockNumber = u32;
/// An alias for a transaction hash type.
pub type TxHash = H256;
/// An alias for the amount of tokens.
pub type Balance = u128;

/// When submitting a transaction, wait for given status before proceeding.
#[derive(Copy, Clone)]
pub enum TxStatus {
    /// A tx must be included in some block.
    InBlock,
    /// A tx must be included in some finalized block.
    Finalized,
    /// A tx must be successfully submitted.
    Submitted,
}

/// Weight of a transaction.
#[derive(Clone, Debug, Eq, PartialEq, codec::Decode, codec::Encode)]
pub struct Weight {
    /// Execution time coordinate.
    #[codec(compact)]
    pub ref_time: u64,
    /// Proof size coordinate.
    #[codec(compact)]
    pub proof_size: u64,
}
