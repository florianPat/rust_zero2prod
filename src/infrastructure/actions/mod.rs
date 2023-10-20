use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::infrastructure::actions::health::health;

mod health;

pub fn actions_factory(app: &mut ServiceConfig) {
    app.route("/health_check", web::get().to(health));
}
