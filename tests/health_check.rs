use std::net::TcpListener;

use emailnewsletter::{run, configuration::{self, get_configuration}};
use sqlx::{PgConnection, Connection};

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    //Act
    let response = client
        .get(format!("{}/health-check", &address))
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
    let address = spawn_app();
    let client = reqwest::Client::new();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    //Connection trait to scope.
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    
    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());
    
    connection.begin();
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
     //Arrange
     let address = spawn_app();
     let client = reqwest::Client::new();
     let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and the email")
     ];

     for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

        //Assert
        assert_eq!(400, response.status().as_u16(), "The API did not fail with bad request when the payload was {}.", error_message);
     }
}
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    let server = emailnewsletter::run(listener).expect("failed to bind address");

    //background task
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)    
}