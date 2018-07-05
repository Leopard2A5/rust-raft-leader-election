extern crate actix;
extern crate actix_web;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate futures;

mod raft_server;
mod messages;

use actix_web::{server::HttpServer, App, http, State};
use std::env;
use raft_server::RaftServer;
use messages::*;
use actix_web::Json;
use actix_web::Responder;
use actix::prelude::*;
use actix::prelude::Addr;

fn append_entries((raft, body): (State<Addr<Syn, RaftServer>>, Json<AppendEntriesRequest>)) -> impl Responder {
    use futures::Future;
    let result = raft.send(body.into_inner());
    let result = result.wait();
    if let Ok(result) = result {
        return Json(result);
    }

    unreachable!();
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let bind = env::var("BIND")
        .ok()
        .unwrap_or("0.0.0.0:8080".into());

    let sys = System::new("foo");
    let raft: Addr<Syn, _> = RaftServer::new().start();
    let _server = HttpServer::new(move ||
        App::with_state(raft.clone())
            .resource("/raft/append-entries", |r| r.method(http::Method::POST)
                .with(append_entries))
        )
        .bind(&bind)
        .unwrap()
        .start();

    let _ = sys.run();
}
