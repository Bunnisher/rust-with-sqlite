use std::{error::Error, result::Result};
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error>{
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = 
    "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS settings
    (
        setting_id      INTEGER PRIMARY KEY NOT NULL,
        description     TEXT                NOT NULL,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        done            BOOLEAN             NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project
    (
        project_id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        first_name                  TEXT,
        last_name                   TEXT,
        company                     TEXT,
        telephone                   TEXT,
        email                       TEXT,
        air_craft                   TEXT,
        serial_num                  TEXT,
        reg_num                     TEXT,
        price                       TEXT,
        status                      TEXT NOT NULL,
        settings_id                 INTEGER NOT NULL DEFAULT 1,
        FOREIGN KEY (settings_id)   REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
    );";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}


#[async_std::main]
async fn main() {
    println!("Hello, Im changed2");
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await{
            Ok(_) => println!("Holy Crap it Worked!!!!"),
            Err(e) => panic!("{}", e)
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    instances.close().await;
    println!("{:?}", result);
}
