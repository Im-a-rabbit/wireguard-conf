#[cfg(feature = "serde")]
mod serde;

mod keys;

use thiserror::Error;

#[cfg(all(feature = "amneziawg-1", feature = "amneziawg-2"))]
compile_error!("Incompatible feature-flags enabled: choose either `amneziawg-1` or `amneziawg-2`");

#[cfg(feature = "amneziawg-1")]
mod amneziawg1;
#[cfg(feature = "amneziawg-1")]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg-1")))]
pub use amneziawg1::*;

#[cfg(feature = "amneziawg-2")]
mod amneziawg2;
#[cfg(feature = "amneziawg-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg-2")))]
pub use amneziawg2::*;

pub use keys::*;

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
    #[cfg(any(feature = "amneziawg-1", feature = "amneziawg-2"))]
    #[error("invalid amnezia setting: {0}")]
    InvalidAmneziaSetting(String),
}

/// Result alias.
pub type WireguardResult<T> = Result<T, WireguardError>;
