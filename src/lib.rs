use std::fmt::Debug;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;

use env_logger::{Builder, WriteStyle};
use env_logger::fmt::Formatter;
use log::{debug, info, Level, LevelFilter, Record};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::{Execute, Executor, QueryBuilder, Row};

use crate::cache_error::{CacheError, MapError};
use crate::crud_ops::{CacheData, create_table, drop_table, insert_into_db, insert_into_db_if_not_exist, print_all_cache, select_all_from_cache, select_from_db};
use crate::map_data::{json_to_obj, obj_to_json};

pub mod cache_error;


fn init_log() {
    let result = Builder::from_default_env()
        .filter(None, LevelFilter::Debug)
        .format(format_log_fn)
        .format_timestamp(None)
        .write_style(WriteStyle::Always)
        .try_init();
    debug!("logger init - {:?}", result)
}

fn format_log_fn(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    if record.level() == Level::Info {
        return Ok(());
    }
    writeln!(buf, "{}", record.args())
}

mod map_data;
mod crud_ops;

/// Main Struct for Cache
#[derive(Debug, Clone)]
pub struct Cache {
    conn_pool: Pool<Sqlite>,
}

impl Cache {
    ///build cache storage
    /// if in_memory = true, will create cache that will be erased after application closed
    /// if in_memory = false, will create a file in the project folder itself to store data
    pub async fn build(in_memory: bool, cache_file_name: &str) -> Self {
        OpenOptions::new()
            .write(true)  // Enable writing to the file.
            .create(true) // Create the file if it doesn't exist.
            .open(format!("{}.db", cache_file_name)).unwrap();

        let conn_pool = if in_memory {
            SqlitePool::connect("sqlite::memory:")
                .await
                .unwrap()
        } else {
            SqlitePool::connect(&format!("sqlite://{}.db", cache_file_name))
                .await
                .unwrap()
        };

        create_table(&conn_pool).await.expect("Could not able to create the Cache table");
        Cache { conn_pool }
    }

    pub async fn build_simple(cache_file_name: Option<String>) -> Result<Cache, CacheError> {
        let conn_pool: Pool<Sqlite> = match cache_file_name {
            None => {
                SqlitePool::connect("sqlite::memory:")
                    .await
                    .unwrap()
            }
            Some(name) => {
                let file_path = format!("{}.db", name);
                let error_msg = format!("Couldn't create or open file: {}", &file_path);
                OpenOptions::new()
                    .write(true)  // Enable writing to the file.
                    .create(true) // Create the file if it doesn't exist.
                    .open(&file_path)
                    .map_to_cache_error(&error_msg)?;

                SqlitePool::connect(&format!("sqlite://{}", file_path))
                    .await
                    .unwrap()
            }
        };
        create_table(&conn_pool).await.expect("Could not able to create the Cache table");
        Ok(Cache { conn_pool })
    }

    /// save object with key, if already exist this will replace
    pub async fn save_obj<T>(&self, key: &str, obj: &T) -> Result<(), CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let content = obj_to_json(obj)?;
        insert_into_db(&self.conn_pool, key, &content).await?;
        Ok(())
    }

    /// save object with key, if already exist this will be ignored
    pub async fn save_obj_if_not_exist<T>(&self, key: &str, obj: &T) -> Result<(), CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let content = obj_to_json(obj)?;
        insert_into_db_if_not_exist(&self.conn_pool, key, &content).await?;
        Ok(())
    }

    /// will retrieve the object for the key
    pub async fn get_obj<T>(&self, key: &str) -> Result<T, CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let cache: CacheData = select_from_db(&self.conn_pool, key).await?;
        let res = json_to_obj::<T>(key, &cache.content)?;
        Ok(res)
    }

    /// get all saved objects from the Cache
    pub async fn get_all_objs(&self) -> Result<Vec<CacheData>, CacheError> {
        let cache_list = select_all_from_cache(&self.conn_pool).await?;
        Ok(cache_list)
    }

    pub async fn pretty_print_all_cache(&self) {
        let cache_list = self.get_all_objs().await.unwrap();
        print_all_cache(&cache_list);
    }
    /// clears all cache data
    pub async fn clear_cache(&self) {
        drop_table(&self.conn_pool).await.expect("Could not drop table to clear previous server session");
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestStruct {
        name: String,
        email: String,
        ph_no: u64,
    }

    #[tokio::test]
    async fn insert_and_retrieve_from_cache() {
        init_log();

        let cache = Cache::build_simple(None).await.unwrap();

        let data = TestStruct {
            name: "dinesh".to_owned(),
            email: "dinesh".to_owned(),
            ph_no: 9999999999u64,
        };

        cache.save_obj("TestData", &data).await.unwrap();

        cache.pretty_print_all_cache();

        let cached_data: TestStruct = cache.get_obj("TestData").await.unwrap();
        debug!("\n\ndeserialized data 'TestStruct' from cache ->\n\n{:?}\n", cached_data);
        cache.clear_cache();
    }

    #[tokio::test]
    async fn test_cache_retrieval() {
        init_log();

        let cache = Cache::build_simple(None).await.unwrap();

        let data = TestStruct {
            name: "dinesh".to_owned(),
            email: "dinesh".to_owned(),
            ph_no: 9999999999u64,
        };

        cache.save_obj("TestData", &data).await.unwrap();

        cache.pretty_print_all_cache();

        let cached_data: TestStruct = cache.get_obj("TestData").await.unwrap();
        debug!("\n\ndeserialized data 'TestStruct' from cache ->\n\n{:?}\n", cached_data);
        cache.clear_cache();
    }
}
