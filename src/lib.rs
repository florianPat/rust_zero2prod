use crate::infrastructure::actions::health::health;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

mod infrastructure;

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
