use core::fmt;

use base64::prelude::*;
use rand::Rng;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::WireguardError;

/// Preshared key.
///
/// Wrapper around `[u8; 32]`.
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
/// // generate new preshared key:
/// let preshared_key = PresharedKey::random();
///
/// // import key:
/// let imported_key = PresharedKey::try_from("MA4zR0tvQpZ4CQ7gs/KcAVMNMGIFBtDcfpjBr+0GwHY=")?;
///
/// // export key via `fmt::Display` trait:
/// let exported_key = imported_key.to_string();
///
/// assert_eq!(exported_key, "MA4zR0tvQpZ4CQ7gs/KcAVMNMGIFBtDcfpjBr+0GwHY=".to_string());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct PresharedKey([u8; 32]);

impl PresharedKey {
    /// Generate random [`PresharedKey`].
    #[must_use]
    pub fn random() -> Self {
        let mut key = [0u8; 32];
        rand::rng().fill_bytes(&mut key);
        Self(key)
    }
}

impl PresharedKey {
    /// Convert [`PresharedKey`] into byte array.
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    /// Get [`PresharedKey`] as reference to bytes.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for PresharedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PresharedKey")
            .field(&self.to_string())
            .finish()
    }
}

/// Export [`PresharedKey`] as base64 format for Wireguard.
impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.as_bytes()))
    }
}

impl From<[u8; 32]> for PresharedKey {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for PresharedKey {
    type Error = WireguardError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPresharedKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPresharedKey)?;

        Ok(Self(bytes))
    }
}

impl TryFrom<String> for PresharedKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::PresharedKey;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

    impl Serialize for PresharedKey {
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

    impl<'de> Deserialize<'de> for PresharedKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let data = String::deserialize(deserializer)?;

                PresharedKey::try_from(data.as_str()).map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(&data), &"a preshared key")
                })
            } else {
                let bytes = <[u8; 32]>::deserialize(deserializer)?;

                Ok(PresharedKey::from(bytes))
            }
        }
    }
}
