use sqlx::mysql::MySqlPool;
use std::env;

pub type DbPool = MySqlPool;

pub async fn establish_connection() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&db_url)
        .await
        .expect("Failed to create MySQL pool")
}
