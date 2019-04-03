use serde::de::DeserializeOwned;

use crate::error::{Result};

pub fn from_str<T>(s: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    //
}