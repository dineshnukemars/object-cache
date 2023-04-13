use std::fmt::Debug;
use std::fmt::format;
use std::fs::OpenOptions;
use std::io::Write;

use env_logger::{Builder, WriteStyle};
use env_logger::fmt::Formatter;
use log::{info, Level, LevelFilter, Record};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::{Execute, Executor, QueryBuilder, Row};

use crate::cache_error::{CacheError, MapError};
use crate::crud_ops::{CacheData, create_table, drop_table, insert_into_db, insert_into_db_if_not_exist, print_all_cache, select_all_from_cache, select_from_db};
use crate::map_data::{json_to_obj, obj_to_json};

pub mod cache_error;


fn init_log() {
    Builder::from_default_env()
        .filter(None, LevelFilter::Debug)
        .format(format_log_fn)
        .format_timestamp(None)
        .write_style(WriteStyle::Always)
        .init();
}

fn format_log_fn(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    if record.level() == Level::Info {
        return Ok(());
    }
    writeln!(buf, "{}", record.args())
}

mod map_data;
mod crud_ops;


#[derive(Debug, Clone)]
pub struct Cache {
    conn_pool: Pool<Sqlite>,
}

impl Cache {
    pub async fn build(in_memory: bool, name: &str) -> Self {
        OpenOptions::new()
            .write(true)  // Enable writing to the file.
            .create(true) // Create the file if it doesn't exist.
            .open(format!("{}.db", name)).unwrap();

        let conn_pool = if in_memory {
            SqlitePool::connect("sqlite::memory:")
                .await
                .unwrap()
        } else {
            SqlitePool::connect(&format!("sqlite://{}.db", name))
                .await
                .unwrap()
        };

        create_table(&conn_pool).await.expect("Could not able to create the Cache table");
        Cache { conn_pool }
    }

    pub async fn save_obj<T>(&self, cache_name: &str, obj: &T) -> Result<(), CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let content = obj_to_json(obj)?;
        insert_into_db(&self.conn_pool, cache_name, &content).await?;
        let cache_list = select_all_from_cache(&self.conn_pool).await?;
        print_all_cache(&cache_list).await?;
        Ok(())
    }


    pub async fn save_obj_if_not_exist<T>(&self, cache_name: &str, obj: &T) -> Result<(), CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let content = obj_to_json(obj)?;
        insert_into_db_if_not_exist(&self.conn_pool, cache_name, &content).await?;
        let cache_list = select_all_from_cache(&self.conn_pool).await?;
        print_all_cache(&cache_list).await?;
        Ok(())
    }


    pub async fn get_obj<T>(&self, cache_name: &str) -> Result<T, CacheError>
        where T: ?Sized + Serialize + DeserializeOwned + Debug {
        let cache: CacheData = select_from_db(&self.conn_pool, cache_name).await?;
        let res = json_to_obj::<T>(cache_name, &cache.content)?;
        Ok(res)
    }

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

        let cache = Cache::build(true, "obj_cache").await;

        let data = TestStruct {
            name: "dinesh".to_owned(),
            email: "dinesh".to_owned(),
            ph_no: 9999999999u64,
        };

        cache.save_obj("TestData", &data).await.unwrap();

        let cached_data: TestStruct = cache.get_obj("TestData").await.unwrap();
        debug!("\n\ndeserialized data 'TestStruct' from cache ->\n\n{:?}\n", cached_data);
        cache.clear_cache();
    }
}
