use std::str::FromStr;

use anyhow::anyhow;
use subxt::{ext::sp_core::Pair, tx::PairSigner, PolkadotConfig};

use crate::{AccountId, RawKeyPair};

/// Used for signing extrinsic payload
pub struct KeyPair {
    inner: PairSigner<PolkadotConfig, RawKeyPair>,
}

impl Clone for KeyPair {
    fn clone(&self) -> Self {
        KeyPair::new(self.inner.signer().clone())
    }
}

impl FromStr for KeyPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let pair = Pair::from_string(s, None)
            .map_err(|e| anyhow!("Can't create pair from seed value: {:?}", e))?;
        Ok(KeyPair::new(pair))
    }
}

impl KeyPair {
    /// Constructs a new KeyPair from RawKeyPair
    pub fn new(keypair: RawKeyPair) -> Self {
        KeyPair {
            inner: PairSigner::new(keypair),
        }
    }

    /// Returns a reference to the inner KeyPair
    pub fn pair_signer(&self) -> &PairSigner<PolkadotConfig, RawKeyPair> {
        &self.inner
    }

    /// Returns a reference to the inner RawKeyPair
    pub fn raw_key_pair(&self) -> &RawKeyPair {
        self.inner.signer()
    }

    /// Returns corresponding AccountId
    pub fn account_id(&self) -> &AccountId {
        self.inner.account_id()
    }
}

/// Converts given seed phrase to a sr25519 [`KeyPair`] object.
/// * `seed` - a 12 or 24 word seed phrase
pub fn keypair_from_string(seed: &str) -> KeyPair {
    KeyPair::new(raw_keypair_from_string(seed))
}

/// Converts given seed phrase to a sr25519 [`RawKeyPair`] object.
/// * `seed` - a 12 or 24 word seed phrase
pub fn raw_keypair_from_string(seed: &str) -> RawKeyPair {
    Pair::from_string(seed, None).expect("Can't create pair from seed value")
}

/// Converts a key pair object to `AccountId`.
/// * `keypair` - a key-pair object, e.g. [`ed25519::Pair`] or [`sr25519::Pair`]
pub fn account_from_keypair<P>(keypair: &P) -> AccountId
where
    P: Pair,
    AccountId: From<<P as Pair>::Public>,
{
    AccountId::from(keypair.public())
}
