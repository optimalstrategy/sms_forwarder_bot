use axum_client_ip::SecureClientIpSource;
use envconfig::Envconfig;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Envconfig, serde_derive::Deserialize, PartialEq, Eq)]
pub struct DbConfig {
    #[envconfig(from = "DB_PORT")]
    pub port: u16,

    #[envconfig(from = "DB_HOST")]
    pub host: String,

    #[envconfig(from = "DB_USER")]
    pub user: String,

    #[envconfig(from = "DB_NAME")]
    pub name: String,

    #[envconfig(from = "DB_PASSWORD")]
    pub password: String,
}

impl DbConfig {
    pub fn connection_string(&self) -> String {
        let Self {
            user,
            password,
            host,
            port,
            name,
        } = self;
        let password = utf8_percent_encode(password, NON_ALPHANUMERIC).to_string();
        format!("postgres://{user}:{password}@{host}:{port}/{name}",)
    }
}

#[derive(Envconfig, serde_derive::Deserialize)]
pub struct AppConfig {
    #[envconfig(nested)]
    pub db: DbConfig,

    #[envconfig(from = "FWD_SERVER_PORT")]
    pub server_port: u16,

    #[envconfig(from = "FWD_SERVER_HOST")]
    pub server_host: String,

    #[envconfig(from = "FWD_IP_SOURCE", default = "XRealIp")]
    pub ip_source: SecureClientIpSource,

    #[envconfig(from = "FWD_BASE_HOST")]
    pub webhook_host: String,

    #[envconfig(from = "FWD_TELEGRAM_TOKEN")]
    pub telegram_token: String,

    #[envconfig(from = "FWD_LONG_POLLING", default = "false")]
    pub use_long_polling: bool,
}
