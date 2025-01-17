use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

use anyhow::Context;
use axum::{Extension, Router};

use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;
use teloxide::update_listeners::{
    webhooks::{self, Options},
    UpdateListener,
};
use teloxide::{prelude::*, Bot};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use url::Url;

use crate::{commands::Command, config::AppConfig};

#[derive(Clone)]
pub struct AppContext {
    pub config: Arc<AppConfig>,
    pub bot: Bot,
    pub db: PgPool,
}

lazy_static::lazy_static! {
    pub static ref WEBHOOK_TOKEN: String = rand::rngs::OsRng.sample_iter(&Alphanumeric)
    .take(48)
    .map(char::from)
    .collect();
}

pub async fn spawn_services(config: AppConfig, bot: Bot, db: PgPool) -> anyhow::Result<()> {
    let config = Arc::new(config);
    let context = AppContext {
        db,
        bot: bot.clone(),
        config: config.clone(),
    };

    let addr = format!("{}:{}", config.server_host, config.server_port)
        .to_socket_addrs()
        .context("Invalid server host or port")?
        .find(|addr| matches!(addr, SocketAddr::V4(_)))
        .context("Could not resolve the server host")?;

    tracing::info!(
        "Configuring the telegram bot (mode = {})",
        if config.use_long_polling {
            "long polling"
        } else {
            "webhook"
        }
    );

    let tcp_listener = TcpListener::bind(addr)
        .await
        .context(format!("Couldn't bind to {addr:?}"))?;
    let mut app = crate::http::api_router();

    if config.use_long_polling {
        let mut listener = teloxide::update_listeners::polling_default(bot).await;
        let stop_token = listener.stop_token();

        app = add_middlewares(app, context.clone());
        tokio::spawn(async move {
            tracing::info!("Starting the server on {addr}");
            axum::serve(
                tcp_listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .inspect_err(|_| stop_token.stop())
            .expect("Axum server error");
        });

        spawn_dispatcher(context, listener).await;
        return Ok(());
    }

    let options = Options::new(
        addr,
        Url::parse(&format!(
            "https://{}/{}",
            config.webhook_host, *WEBHOOK_TOKEN
        ))
        .context("Invalid webhook host")?,
    );
    let (mut listener, stop_flag, router) = webhooks::axum_to_router(bot, options).await?;
    app = app.merge(router);
    app = add_middlewares(app, context.clone());

    let stop_token = listener.stop_token();
    tokio::spawn(async move {
        tracing::info!("Starting the server on {addr}");
        axum::serve(
            tcp_listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(stop_flag)
        .await
        .inspect_err(|_| stop_token.stop())
        .expect("Axum server error");
    });

    spawn_dispatcher(context, listener).await;
    Ok(())
}

async fn spawn_dispatcher<UListener>(context: AppContext, update_listener: UListener)
where
    UListener: UpdateListener,
    UListener::Err: std::fmt::Debug,
{
    Dispatcher::builder(
        context.bot.clone(),
        Update::filter_message().branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(move |_: Bot, msg, cmd| {
                    crate::handlers::handle(context.clone(), msg, cmd)
                }),
        ),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch_with_listener(update_listener, LoggingErrorHandler::new())
    .await;
}

fn add_middlewares(router: Router, context: AppContext) -> Router {
    router
        .layer(context.config.ip_source.clone().into_extension())
        .layer(Extension(context))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
