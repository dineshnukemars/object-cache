use std::fmt::Debug;

use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{Pool, Sqlite};
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

pub async fn init_db(connection_pool: &Pool<Sqlite>) {
    drop_table(connection_pool).await.expect("Could not drop table to clear previous server session");
    create_table(connection_pool).await.expect("Could not able to create the Cache table");
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


#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::app_db::crud_ops::create_table;
    use crate::session::ClientSessionData;
    use crate::utils::init_log;

    use super::*;

    #[tokio::test]
    async fn insert_and_retrieve_from_cache() {
        init_log();

        let connection_pool: Pool<Sqlite> = SqlitePool::connect("sqlite::memory:").await.unwrap();
        create_table(&connection_pool).await.unwrap();

        let session_data = ClientSessionData::new("112", "Medi");
        session_data.save_to_cache(&connection_pool);
        save(&connection_pool, &session_data).await.unwrap();

        let cache: ClientSessionData = get(&connection_pool).await.unwrap();
        debug!("\n\ndeserialized data 'ClientSessionData' from cache ->\n\n{:?}\n", cache);
    }
}