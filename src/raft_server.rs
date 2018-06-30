use std::sync::RwLock;
use messages::{AppendEntriesRequest, AppendEntriesResponse};
use std::cmp;
use std::fs::File;
use std::io::Write;
use serde_json;

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
        let term_response;
        {
            let mut persistent_state = self.persistent_state.write().unwrap();

            persistent_state.current_term = cmp::max(persistent_state.current_term, message.term);
            term_response = persistent_state.current_term;

            let mut file = File::create("raft_persistent_state.json").unwrap();
            let json = serde_json::to_string(&*persistent_state).unwrap();
            file.write(json.as_bytes()).unwrap();
        }

        AppendEntriesResponse {
            term: term_response,
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
