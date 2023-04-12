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


#[cfg(test)]
mod tests {
    use log::debug;
    use serde::Deserialize;
    use sqlx::SqlitePool;

    use crate::app_db::crud_ops::create_table;
    use crate::init_log;

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestStruct {
        name: String,
        email: String,
        ph_no: u64,
    }

    impl DbCache for TestStruct {
        fn get_cache_name() -> String {
            "TestStruct".to_owned()
        }
    }

    #[tokio::test]
    async fn insert_and_retrieve_from_cache() {
        init_log();

        // let connection_pool: Pool<Sqlite> = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let conn_pool = init_db("test_cache").await;
        create_table(&conn_pool).await.unwrap();

        let data = TestStruct {
            name: "dinesh".to_owned(),
            email: "dinesh".to_owned(),
            ph_no: 9999999999u64,
        };

        save(&conn_pool, &data).await.unwrap();

        let cache: TestStruct = get(&conn_pool).await.unwrap();
        debug!("\n\ndeserialized data 'TestStruct' from cache ->\n\n{:?}\n", cache);
    }
}