#[cfg(feature = "serde")]
mod serde;

mod keys;
pub use keys::*;

use thiserror::Error;

/// `wireguard-conf` error.
#[derive(Error, Debug, PartialEq)]
pub enum WireguardError {
    /// Error, when private key is invalid.
    #[error("invalid private key")]
    InvalidPrivateKey,

    /// Error, when public key is invalid.
    #[error("invalid public key")]
    InvalidPublicKey,

    /// Error, when preshared key is invalid.
    #[error("invalid preshared key")]
    InvalidPresharedKey,

    /// Error, when private key isn't provided.
    #[error("no private key provided")]
    NoPrivateKeyProvided,

    /// Error, when no IP assigned.
    #[error("no assigned ip")]
    NoAssignedIP,

    /// Error, when some amnezia setting is invalid
    #[cfg(feature = "amneziawg")]
    #[error("invalid amnezia setting: {0}")]
    InvalidAmneziaSetting(String),
}

/// Result alias.
pub type WireguardResult<T> = Result<T, WireguardError>;
