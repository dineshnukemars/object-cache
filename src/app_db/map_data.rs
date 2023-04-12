use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::app_db::DbCache;
use crate::app_error::{AppError, MapError};

pub fn obj_to_json<T>(obj: &T) -> Result<String, AppError>
    where T: ?Sized + Serialize + Debug {
    let err_msg = format!("Could not serialize obj : {:?}", obj);
    let json = serde_json::to_string(obj).map_to_app_error(&err_msg)?;
    Ok(json)
}

pub fn json_to_obj<T>(json: &str) -> Result<T, AppError>
    where T: ?Sized + DeserializeOwned + DbCache {
    let cache_name = T::get_cache_name();
    let err_msg = format!("Could not deserialize to Type: {} \n for the json: {}", &cache_name, &json);
    let obj = serde_json::from_str::<T>(json).map_to_app_error(&err_msg)?;
    Ok(obj)
}