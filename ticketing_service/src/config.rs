pub use config::ConfigError;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub db_connection_string: String,
    pub main_service_host: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub db_pool: MySqlPool,
    pub http_client: reqwest::Client,
    pub main_service_host: String,
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

        // get envars

        let main_service_host = config.server.main_service_host;

        // Initialize http client
        let client = reqwest::Client::new();

        let app_config = AppConfig {
            db_pool: new_db_pool,
            http_client: client,
            main_service_host: main_service_host,
        };

        return app_config;
    }
}
