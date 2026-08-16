use core::fmt;

use base64::prelude::*;
use x25519_dalek::StaticSecret;
use zeroize::ZeroizeOnDrop;

use crate::WireguardError;

/// Private key
///
/// Wrapper around [`x25519_dalek::StaticSecret`] with some traits.
///
/// # Implements
///
/// - Implements [`Zeroize`] and [`ZeroizeOnDrop`] for clearing secrets from memory.
/// - Implements [`TryFrom<&str>`] or [`TryFrom<String>`] for importing key from Base64 format.
/// - Implements [`fmt::Display`] for exporting key in Wireguard's format.
/// - Implements [`fmt::Debug`].
///
/// # Examples
///
/// ```
/// # use wireguard_conf::prelude::*;
/// # fn main() -> WireguardResult<()> {
/// // generate new random key:
/// let key = PrivateKey::random();
///
/// // import key:
/// let imported_key = PrivateKey::try_from("sJkP2oorqrq49P6Ln25MWo3X04PxhB8k+RnJJnZ4gEo=")?;
///
/// // export key via `fmt::Display` trait:
/// let exported_key = imported_key.to_string();
///
/// assert_eq!(exported_key, "sJkP2oorqrq49P6Ln25MWo3X04PxhB8k+RnJJnZ4gEo=".to_string());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, ZeroizeOnDrop)]
pub struct PrivateKey(pub(crate) StaticSecret);

impl PrivateKey {
    /// Generate new a random [`PrivateKey`]
    #[must_use]
    pub fn random() -> PrivateKey {
        Self(StaticSecret::random())
    }
}

impl PrivateKey {
    /// View private key as byte array.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    /// Convert private key to a byte array.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PrivateKey")
            .field(&self.to_string())
            .finish()
    }
}

/// Export key as base64 for Wireguard.
impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.as_bytes()))
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl TryFrom<&str> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPrivateKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPrivateKey)?;

        Ok(Self(StaticSecret::from(bytes)))
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<[u8; 32]> for PrivateKey {
    fn from(value: [u8; 32]) -> Self {
        Self(StaticSecret::from(value))
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::PrivateKey;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

    impl Serialize for PrivateKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&self.to_string())
            } else {
                serializer.serialize_bytes(self.as_bytes())
            }
        }
    }

    impl<'de> Deserialize<'de> for PrivateKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let data = String::deserialize(deserializer)?;

                PrivateKey::try_from(data.as_str()).map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(&data), &"a private key")
                })
            } else {
                let bytes = <[u8; 32]>::deserialize(deserializer)?;

                Ok(PrivateKey::from(bytes))
            }
        }
    }
}
