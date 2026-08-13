mod ver1;
mod ver2;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

pub use ver1::*;
pub use ver2::*;

use crate::WireguardResult;

/// [AmneziaWG](https://docs.amnezia.org/documentation/amnezia-wg) obfuscation values. By using this
/// enum, you can parse, generate and validate for all AmneziaWG versions.
///
/// ```
/// # use wireguard_conf::prelude::*;
/// let settings = AmneziaWG::random_v1();
/// let settings = AmneziaWG::random_v2();
///
/// assert_eq!(settings.validate(), Ok(()));
///
/// // With `serde` feature you can serialize and deserialize obfuscation values.
///
/// println!("{settings}");
/// ```
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AmneziaWG {
    /// AmneziaWG 1.0
    V1(AmneziaWG1),

    /// AmneziaWG 2.0
    V2(AmneziaWG2),
}

impl AmneziaWG {
    /// Generate random recommended settings for **AmneziaWG 1.0**.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaWG::random_v1();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn random_v1() -> Self {
        Self::V1(AmneziaWG1::random())
    }

    /// Generate random recommended settings for **AmneziaWG 2.0**.
    ///
    /// # Examples
    ///
    /// ```
    /// use wireguard_conf::prelude::*;
    ///
    /// let settings = AmneziaWG::random_v2();
    ///
    /// _ = InterfaceBuilder::new()
    ///    // <snip>
    ///    .amnezia_settings(settings)
    ///    .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn random_v2() -> Self {
        AmneziaWG2::random()
    }

    /// Alias for `AmneziaWG::builder_v2().build()`
    #[inline]
    #[must_use]
    pub fn empty_v2() -> Self {
        Self::V2(AmneziaWG2::default())
    }

    /// Create new [`AmneziaWG2Builder`].
    ///
    /// This function is an alias for [`AmneziaWG2::builder()`].
    #[inline]
    #[must_use]
    pub fn builder_v2() -> AmneziaWG2Builder {
        AmneziaWG2::builder()
    }

    /// Validates [`AmneziaWG`] values.
    ///
    /// ```
    /// # use wireguard_conf::prelude::*;
    /// let settings = AmneziaWG::random_v2();
    ///
    /// assert_eq!(settings.validate(), Ok(()));
    /// ```
    ///
    /// # Errors
    ///
    /// If [`AmneziaSettings`] is invalid, it will throw [`WireguardError::InvalidAmneziaSetting`]
    /// with setting name
    pub fn validate(&self) -> WireguardResult<()> {
        match self {
            AmneziaWG::V1(settings) => settings.validate(),
            AmneziaWG::V2(settings) => settings.validate(),
        }
    }

    /// Removes server-only AmneziaWG settings.
    ///
    /// This is intended for generating client configuration files, where
    /// server-specific fields (such as I1-I5) must not be included.
    pub fn strip_server_data(&mut self) {
        match self {
            AmneziaWG::V1(_) => {}
            AmneziaWG::V2(settings) => settings.strip_server_data(),
        }
    }
}

/// Implementation of [`fmt::Display`] for exporting [`AmneziaWG`] values.
///
/// # Note
///
/// It exports only `Jc`, `Jmin`, etc.
///
/// To export full interface, use [`Interface::to_string()`].
impl fmt::Display for AmneziaWG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmneziaWG::V1(settings) => settings.fmt(f),
            AmneziaWG::V2(settings) => settings.fmt(f),
        }
    }
}
