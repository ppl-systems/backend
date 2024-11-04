use crate::routes::generate;
use actix_cors::Cors;
use actix_web::{dev::Server, http, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server: Server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["POST", "OPTIONS", "GET"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE]),
            )
            .wrap(TracingLogger::default())
            .service(generate)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
