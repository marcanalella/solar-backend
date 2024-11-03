use solar_backend::configuration::get_configuration;
use solar_backend::telemetry::{get_subscriber, init_subscriber};
use solar_backend::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("solar_backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}