#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "json")]
use serde_json::{from_str, to_string, Result as SerdeResult};

/// Trait that [`serialize`] and [`deserialize`] from/to json
///
/// [`serialize`]: Serialize
/// [`deserialize`]: Deserialize
#[cfg(feature = "json")]
pub trait ToFromJson<'de>
where
    Self: Sized + Serialize + Deserialize<'de>
{
    /// [`Deserialize`] struct from json with [`serde_json::from_str`]
    fn from_json(json: &'de str) -> SerdeResult<Self> {
        from_str(json)
    }

    /// [`Serialize`] struct to json with [`serde_json::to_string`]
    fn to_json(&self) -> SerdeResult<String> {
        to_string(self)
    }
}
