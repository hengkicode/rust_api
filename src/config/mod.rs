use sqlx::mysql::MySqlPool;
use std::env;
use dotenv::dotenv;

pub fn init_config() {
    dotenv().ok(); // Memuat variabel lingkungan dari file .env
}

pub async fn init_db() -> MySqlPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Membuat pool koneksi ke MySQL
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Error creating database pool");

    pool
}
