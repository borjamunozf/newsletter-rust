use std::net::TcpListener;

use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{PgConnection, PgPool};
use tracing_actix_web::TracingLogger;

use crate::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    //smart pointer
    //shared mutable state by all workers
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health-check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
