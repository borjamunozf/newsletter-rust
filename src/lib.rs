use actix_web::{HttpServer, Responder, web, App, HttpResponse, dev::Server};

async fn health_check() -> impl Responder {
    // todo!()
    HttpResponse::Ok()   
 }
 
 pub fn run() -> Result<Server, std::io::Error> {
     let server = HttpServer::new(|| {
         App::new()
             .route("/health-check", web::get().to(health_check))
     })
     .bind("127.0.0.1:8000")?
     .run();

     Ok(server)
 }