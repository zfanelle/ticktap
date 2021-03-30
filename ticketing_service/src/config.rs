pub use config::ConfigError;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub db_connection_string: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_pool: MySqlPool,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    pub async fn initialize_application() -> AppConfig {
        // Initialize logger
        env_logger::init();

        // Initialize database pool into static TypeMap
        let config = Config::from_env().unwrap();

        let db_connection_string = config.server.db_connection_string.to_owned();

        let db_pool = MySqlPool::connect(&db_connection_string).await;

        let new_db_pool = db_pool.unwrap();

        let app_config = AppConfig {
            db_pool: new_db_pool,
        };

        return app_config;
    }
}
