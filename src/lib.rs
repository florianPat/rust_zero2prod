use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use crate::infrastructure::actions::actions_factory;

mod infrastructure;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().configure(actions_factory))
        .listen(listener)?
        .run();

    Ok(server)
}
