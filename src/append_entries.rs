use ::Term;
use raft_server::LogEntry;
use serde_json;
use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use actix::prelude::Message;
use actix::dev::MessageResponse;
use raft_server::RaftServer;
use actix::dev::ResponseChannel;
use actix::Actor;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppendEntriesRequest {
    pub term: Term,
    pub leader_id: String,
    pub prev_log_index: Term,
    pub prev_log_term: Term,
    pub entries: Vec<LogEntry>,
    pub leader_commit: Term
}

impl Message for AppendEntriesRequest {
    type Result = AppendEntriesResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: Term,
    pub success: bool
}

impl MessageResponse<RaftServer, AppendEntriesRequest> for AppendEntriesResponse {
    fn handle<R: ResponseChannel<AppendEntriesRequest>>(self, _ctx: &mut <RaftServer as Actor>::Context, tx: Option<R>) {
        if let Some(chan) = tx {
            chan.send(self)
        }
    }
}

impl Responder for AppendEntriesResponse {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        let body = serde_json::to_string(&self)?;
        Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        )
    }
}
