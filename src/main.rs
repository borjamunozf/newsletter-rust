use std::net::TcpListener;

use emailnewsletter::{run, configuration::get_configuration};
use sqlx::PgConnection;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuratin");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind port");

    run(listener, connection)?.await
}
