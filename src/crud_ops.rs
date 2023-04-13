use log::debug;
use sqlx::{Executor, Pool, Sqlite};

use crate::cache_error::{CacheError, MapError};

const TABLE_NAME: &str = "cache";

#[derive(Debug, sqlx::FromRow)]
pub struct CacheData {
    pub name: String,
    pub content: String,
}

pub async fn drop_table(connection_pool: &Pool<Sqlite>) -> Result<(), CacheError> {
    let sql = format!("DROP TABLE IF EXISTS {TABLE_NAME:?}");
    connection_pool
        .execute(sql.as_str())
        .await
        .map_to_cache_error(&format!("Could not Drop DB table {:?}", TABLE_NAME))?;
    debug!("Table {} dropped successfully!😢", TABLE_NAME);
    Ok(())
}

pub async fn create_table(connection_pool: &Pool<Sqlite>) -> Result<(), CacheError> {
    let create_table_query = format!("CREATE TABLE IF NOT EXISTS {} (name text PRIMARY KEY,content text);", TABLE_NAME);
    debug!("\n\nquery -> {}",create_table_query);

    sqlx::query(&create_table_query)
        .execute(connection_pool)
        .await
        .map_to_cache_error("Could not create table")?;

    debug!("{} table created!! 😀\n",TABLE_NAME);
    Ok(())
}

pub async fn insert_into_db(
    connection_pool: &Pool<Sqlite>,
    cache_name: &str,
    content: &str,
) -> Result<(), CacheError> {
    let insert_query = format!("INSERT INTO {TABLE_NAME}(name, content) values ('{cache_name}','{content}') on conflict(name) do update set content=excluded.content");
    debug!("\nQuery -> {}",insert_query);

    sqlx::query(&insert_query)
        .execute(connection_pool)
        .await
        .map_to_cache_error("Could not insert into DB!")?;

    debug!("data inserted! 😇\n");
    Ok(())
}

pub async fn insert_into_db_if_not_exist(connection_pool: &Pool<Sqlite>, cache_name: &str, content: &str) -> Result<(), CacheError> {
    let query = format!("INSERT OR IGNORE INTO {TABLE_NAME:?}(name, content) values ('{}','{}') on conflict(name) do update set content=excluded.content", &cache_name, &content);
    debug!("\nQuery -> {}",query);

    sqlx::query(&query)
        .execute(connection_pool)
        .await
        .map_to_cache_error("Could not insert into DB!")?;

    debug!("data inserted! 😇\n");
    Ok(())
}

pub async fn select_from_db(connection_pool: &Pool<Sqlite>, cache_name: &str) -> Result<CacheData, CacheError> {
    let query = format!("SELECT * FROM {TABLE_NAME:?} where name = '{}'", cache_name);
    debug!("Query -> {}",query);

    let cache: CacheData = sqlx::query_as(&query)
        .fetch_one(connection_pool)
        .await
        .map_to_cache_error(&format!("Could not retrieve cache for name {}", cache_name))?;

    debug!("content from db for cache:🧐 {} ->\n\n{:?}", cache_name, cache);
    Ok(cache)
}

pub async fn select_all_from_cache(connection_pool: &Pool<Sqlite>) -> Result<Vec<CacheData>, CacheError> {
    let query = format!("SELECT * FROM {:?}", TABLE_NAME);
    debug!("\nQuery -> {}\n",query);

    let list_of_rows: Vec<CacheData> = sqlx::query_as(&query)
        .fetch_all(connection_pool)
        .await
        .map_to_cache_error("Could not retrieve data from Database")?;

    Ok(list_of_rows)
}


pub async fn print_all_cache(list_of_rows: &Vec<CacheData>) -> Result<(), CacheError> {
    const NAME_HEADER: &str = "Name";
    const CONTENT_HEADER: &str = "Content";

    // Calculate column widths
    let name_width = list_of_rows.iter().map(|u| u.name.len()).max().unwrap_or(NAME_HEADER.len());
    let content_width = list_of_rows.iter().map(|u| u.content.to_string().len()).max().unwrap_or(CONTENT_HEADER.len());

    // Print header
    debug!("{:<width$} | {:<width$}",
             NAME_HEADER,
             CONTENT_HEADER,
             width = name_width.max(content_width),
    );

    // Print separator
    debug!("{:-<1$}-+-{:-<2$}",
             "",
             name_width,
             content_width,
    );

    // Print rows
    for cache in list_of_rows {
        debug!("{:<name_width$} | {:<age_width$}",
                 cache.name,
                 cache.content,
                 name_width = name_width,
                 age_width = content_width,
        );
    }
    debug!("\nEnd----------------------🧐\n");
    Ok(())
}


#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use crate::init_log;

    use super::*;

    #[tokio::test]
    async fn test_db_create_table() {
        init_log();

        let connection_pool: Pool<Sqlite> = SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();

        create_table(&connection_pool).await.unwrap();
        insert_into_db(&connection_pool, "Dinesh_Cache", r#"dummy content: {"test":"data"}"#).await.unwrap();

        let cache_list = select_all_from_cache(&connection_pool).await.unwrap();
        print_all_cache(&cache_list).await.unwrap();
    }
}