use std::fmt::{Debug, format};
use std::fs::OpenOptions;

use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::{Execute, Executor, QueryBuilder, Row};

use crate::app_db::crud_ops::{create_table, drop_table};
use crate::app_db::crud_ops::{CacheData, insert_into_db, insert_into_db_if_not_exist, print_all_cache, select_all_from_cache, select_from_db};
use crate::app_db::map_data::{json_to_obj, obj_to_json};
use crate::app_error::{AppError, MapError};

mod map_data;
mod crud_ops;


pub trait DbCache {
    fn get_cache_name() -> String;
}

pub async fn init_db(name: &str) -> Pool<Sqlite> {
    OpenOptions::new()
        .write(true)  // Enable writing to the file.
        .create(true) // Create the file if it doesn't exist.
        .open(format!("{}.db",name)).unwrap();

    let db_url = format!("sqlite://{}.db", name);

    let conn_pool = SqlitePool::connect(&db_url)
        .await
        .unwrap();

    create_table(&conn_pool).await.expect("Could not able to create the Cache table");
    conn_pool
}

pub async fn clear_db(conn_pool: &Pool<Sqlite>) {
    drop_table(conn_pool).await.expect("Could not drop table to clear previous server session");
}

pub async fn save<T>(connection_pool: &Pool<Sqlite>, obj: &T) -> Result<(), AppError>
    where T: ?Sized + Serialize + DeserializeOwned + Debug + DbCache {
    let cache_name = T::get_cache_name();
    let content = obj_to_json(obj)?;
    insert_into_db(&connection_pool, &cache_name, &content).await?;
    let cache_list = select_all_from_cache(connection_pool).await?;
    print_all_cache(&cache_list).await?;
    Ok(())
}

pub async fn save_if_not_exist<T>(connection_pool: &Pool<Sqlite>, obj: &T) -> Result<(), AppError>
    where T: ?Sized + Serialize + DeserializeOwned + Debug + DbCache {
    let cache_name = T::get_cache_name();
    let content = obj_to_json(obj)?;
    insert_into_db_if_not_exist(&connection_pool, &cache_name, &content).await?;
    let cache_list = select_all_from_cache(connection_pool).await?;
    print_all_cache(&cache_list).await?;
    Ok(())
}

pub async fn get<T>(connection_pool: &Pool<Sqlite>) -> Result<T, AppError>
    where T: ?Sized + Serialize + DeserializeOwned + Debug + DbCache {
    let cache_name = T::get_cache_name();
    let cache: CacheData = select_from_db(&connection_pool, &cache_name).await?;
    let res = json_to_obj::<T>(&cache.content)?;
    Ok(res)
}