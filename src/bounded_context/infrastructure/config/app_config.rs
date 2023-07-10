use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub db_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        load_config()
    }
}

pub fn load_config() -> AppConfig {
    dotenv().ok();

    let db_url = env::var("DB_URL")
        .expect("DB_URL not set in .env file");

    AppConfig {
        db_url,
    }
}
