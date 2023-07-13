use std::env;

use sea_orm::{Database, DatabaseConnection};

pub mod models;

pub async fn get_pool() -> DatabaseConnection {
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match Database::connect(database_url).await {
        Ok(connection) => connection,
        Err(e) => {
            panic!("Error connecting to database: {}", e);
        }
    }
}
