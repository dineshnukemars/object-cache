use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cache_error::{CacheError, MapError};

pub fn obj_to_json<T>(obj: &T) -> Result<String, CacheError>
    where T: ?Sized + Serialize + Debug {
    let err_msg = format!("Could not serialize obj : {:?}", obj);
    let json = serde_json::to_string(obj).map_to_cache_error(&err_msg)?;
    Ok(json)
}

pub fn json_to_obj<T>(cache_name: &str, json: &str) -> Result<T, CacheError>
    where T: ?Sized + DeserializeOwned {
    let err_msg = format!("Could not deserialize to Type: {} \n for the json: {}", &cache_name, &json);
    let obj = serde_json::from_str::<T>(json).map_to_cache_error(&err_msg)?;
    Ok(obj)
}