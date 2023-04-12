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

pub fn parse_json<T>(json_str: &str) -> Result<T, AppError> where T: Debug + DeserializeOwned {
    let err_msg = "Error while converting json to Object type";
    let result: T = serde_json::from_str::<T>(&json_str).map_to_app_error(err_msg)?;
    info!("{:?}",&result);
    Ok(result)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
