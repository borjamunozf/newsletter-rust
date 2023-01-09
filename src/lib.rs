use std::net::TcpListener;

use actix_web::{HttpServer, Responder, web, App, HttpResponse, dev::Server};
use sqlx::PgConnection;

pub mod configuration;
pub mod routes;
pub mod startup;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    //smart pointer
    //shared mutable state by all workers
    let connection = web::Data::new(connection);

     let server = HttpServer::new(move || {
         App::new()
             .route("/health-check", web::get().to(routes::health_check))
             .route("/subscriptions", web::post().to(routes::subscribe))
             .app_data(connection.clone())
     })
     .listen(listener)?
     .run();

     Ok(server)
}