use emailnewsletter::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    //Act
    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request");
        
    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = emailnewsletter::run().expect("failed to bind address");

    //background task
    let _ = tokio::spawn(server);
}