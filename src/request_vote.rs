use ::Term;
use ::LogIndex;
use raft_server::RaftServer;
use actix::Actor;
use actix::dev::MessageResponse;
use actix::Message;
use actix::dev::ResponseChannel;
use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RequestVoteRequest {
    pub term: Term,
    pub candidate_id: String,
    pub last_log_index: LogIndex,
    pub last_log_term: LogIndex
}

impl Message for RequestVoteRequest {
    type Result = RequestVoteResponse;
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RequestVoteResponse {
    pub term: Term,
    pub vote_granted: bool
}

impl MessageResponse<RaftServer, RequestVoteRequest> for RequestVoteResponse {
    fn handle<R: ResponseChannel<RequestVoteRequest>>(self, _ctx: &mut <RaftServer as Actor>::Context, tx: Option<R>) {
        if let Some(chan) = tx {
            chan.send(self)
        }
    }
}

impl Responder for RequestVoteResponse {
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
