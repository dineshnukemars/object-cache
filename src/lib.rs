use std::fmt::Debug;
use std::io::Write;
use env_logger::{Builder, WriteStyle};
use env_logger::fmt::Formatter;
use log::{info, Level, LevelFilter, Record};
use serde::de::DeserializeOwned;
use crate::app_error::{AppError, MapError};

pub mod app_db;
pub mod app_error;


pub fn init_log() {
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use log::debug;
    use serde::{Deserialize, Serialize};
    use crate::app_db::{DbCache, get_obj, init_db, save_or_replace};
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
        let conn_pool = init_db("obj_cache").await;

        let data = TestStruct {
            name: "dinesh".to_owned(),
            email: "dinesh".to_owned(),
            ph_no: 9999999999u64,
        };

        save_or_replace(&conn_pool, &data).await.unwrap();

        let cache: TestStruct = get_obj(&conn_pool).await.unwrap();
        debug!("\n\ndeserialized data 'TestStruct' from cache ->\n\n{:?}\n", cache);
    }
}
