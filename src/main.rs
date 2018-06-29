extern crate actix_web;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;

use actix_web::{server,App,HttpRequest};
use std::env;

fn index(_req: HttpRequest) -> &'static str {
    "Hi!"
}

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let bind = env::var("BIND")
        .ok()
        .unwrap_or("0.0.0.0:8080".into());

    info!("Listening on {}", bind);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(&bind)
        .unwrap()
        .run();
}
