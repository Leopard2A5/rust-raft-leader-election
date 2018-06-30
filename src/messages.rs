use raft_server::LogEntry;
use raft_server::RaftServer;
use std::sync::Arc;
use serde_json;
use actix_web::{Responder, Error, HttpRequest, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    pub term: i32,
    pub leader_id: String,
    pub prev_log_index: i32,
    pub prev_log_term: i32,
    pub entries: Vec<LogEntry>,
    pub leader_commit: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: i32,
    pub success: bool
}

impl Responder for AppendEntriesResponse {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        let body = serde_json::to_string(&self)?;
        Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        )
    }
}
