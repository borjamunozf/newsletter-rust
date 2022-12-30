use emailnewsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}


#[cfg(tests)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeded() {
        let response = health_check().await;

        assert!(response.status().is_success());
    }
}