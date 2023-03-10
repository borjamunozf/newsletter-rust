use std::net::TcpListener;

use emailnewsletter::configuration::get_configuration;
use emailnewsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuratin");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind port");

    emailnewsletter::startup::run(listener, connection_pool)?.await
}
