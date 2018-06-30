use std::sync::RwLock;
use messages::{AppendEntriesRequest, AppendEntriesResponse};

#[derive(Debug)]
pub struct RaftServer {
    persistent_state: RwLock<PersistentState>
}

impl RaftServer {
    pub fn new() -> Self {
        RaftServer {
            persistent_state: RwLock::new(PersistentState {
                current_term: 0,
                voted_for: None,
                log: vec![]
            })
        }
    }

    pub fn append_entries(&self, message: AppendEntriesRequest) -> AppendEntriesResponse {
//        self.persistent_state.write().unwrap().current_term += 1;
        AppendEntriesResponse {
            term: 1,
            success: true
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistentState {
    current_term: i32,
    voted_for: Option<String>,
    log: Vec<LogEntry>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogEntry {

}
