use std::net::TcpListener;

use emailnewsletter::{
    configuration::{self, get_configuration, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};

use once_cell::sync::Lazy;
use reqwest::get;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    //Act
    let response = client
        .get(format!("{}/health-check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let configuration = get_configuration().expect("Failed to read configuration");

    //Connection trait to scope.
    let connection = PgConnection::connect_with(&configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and the email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with bad request when the payload was {}.",
            error_message
        );
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

//The init trace subscriber will be called for all our tests and it must be called once.
//Using crate::once_cell
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    //background task
    let server = emailnewsletter::startup::run(listener, connection_pool.clone())
        .expect("failed to bind address");
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE_DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
