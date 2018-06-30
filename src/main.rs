extern crate actix_web;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod raft_server;
mod messages;

use actix_web::{server,App,http};
use std::env;
use raft_server::RaftServer;
use messages::*;
use std::sync::Arc;
use actix_web::Json;
use actix_web::State;

fn append_entries((raft_server, body): (State<Arc<RaftServer>>, Json<AppendEntriesRequest>)) -> String {
    serde_json::to_string(
        &raft_server.append_entries(body.into_inner())
    ).unwrap()
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
            .resource("/raft/append-entries", |r| r.method(http::Method::POST).with(append_entries) ))
        .bind(&bind)
        .unwrap()
        .run();
}
