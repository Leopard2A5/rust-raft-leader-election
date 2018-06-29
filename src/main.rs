extern crate actix_web;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod raft_server;

use actix_web::{server,App,HttpRequest};
use std::env;
use raft_server::RaftServer;
use std::sync::Arc;

fn index(_req: HttpRequest<Arc<RaftServer>>) -> &'static str {
    _req.state().increment_term();
    "Hi!"
}

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let bind = env::var("BIND")
        .ok()
        .unwrap_or("0.0.0.0:8080".into());

    let raft_server = Arc::new(RaftServer::new());

    server::new(move ||
        App::with_state(raft_server.clone())
            .resource("/", |r| r.f(index)))
        .bind(&bind)
        .unwrap()
        .run();
}
