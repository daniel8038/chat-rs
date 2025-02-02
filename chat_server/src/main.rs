use anyhow::Result;
use chat::{config::AppConfig, get_router};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    // log
    let fmt_layer = Layer::new()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_thread_names(true)
        .pretty()
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(fmt_layer).init();
    // server
    let config = AppConfig::load()?;
    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    let app = get_router(config);
    axum::serve(listener, app.into_make_service()).await?;
    info!("setup server on {}", addr);
    Ok(())
}
