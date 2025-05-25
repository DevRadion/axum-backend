use dotenvy::dotenv;
use {{crate_name}}::{
    infrastracture::config::{AppConfig, ConfigError},
    presentation::routes::create_router,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    let app = create_router();
    let config = load_config()?;

    let address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&address).await?;
    info!("Starting the server at: {}", address);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::fmt::init();
}

fn load_config() -> Result<AppConfig, ConfigError> {
    dotenv().ok();

    AppConfig::from_env()
}
