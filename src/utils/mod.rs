#[cfg(feature = "serde")]
mod serde;

mod keys;

use thiserror::Error;

// There's a lot of `cfg`, what they basicly do:
//
//   1. You cannot enable two or more `amneziawg-*` together
//   2. Based of what `amneziawg-*` you enabled, required module will be enabled
//   3. If code builds for documentation, linting or testing, then first check is disabled and AmneziaWG 2.0 is imported as default

#[cfg(all(
    feature = "amneziawg-1",
    feature = "amneziawg-2",
    not(feature = "__amneziawg_internal"),
))]
compile_error!("Incompatible feature-flags enabled: choose either `amneziawg-1` or `amneziawg-2`");

#[cfg(feature = "amneziawg-1")]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg-1")))]
pub mod amneziawg1;

#[cfg(feature = "amneziawg-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg-2")))]
pub mod amneziawg2;

#[cfg(all(feature = "amneziawg-1", any(doc, not(feature = "__amneziawg_internal"))))]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg-1")))]
pub use amneziawg1::*;
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
