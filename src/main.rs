use solar_backend::configuration::get_configuration;
use solar_backend::telemetry::{get_subscriber, init_subscriber};
use solar_backend::startup::run;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("solar_backend".into(), "info".into(), std::io::sink);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // No longer async, given that we don't actually try to connect!
    let connection_pool = PgPool::connect_lazy(
        &configuration.database.connection_string().expose_secret()
    ).expect("Failed to create Postgres connection pool.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}