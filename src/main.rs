mod commands;
mod config;
mod handlers;
mod http;
mod queries;
mod setup;

use std::time::Duration;

use config::AppConfig;
use envconfig::Envconfig;
use sqlx::postgres::PgPoolOptions;
use teloxide::prelude::*;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::init_from_env()?;
    let bot = Bot::new(&config.telegram_token);
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(15))
        .connect(&config.db.connection_string())
        .await
        .expect("can't connect to database");

    tracing::info!("Migrating the database");
    sqlx::migrate!().run(&pool).await?;

    setup::spawn_services(config, bot, pool).await?;
    Ok(())
}
