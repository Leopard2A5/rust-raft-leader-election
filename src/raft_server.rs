use std::sync::RwLock;
use messages::{AppendEntriesRequest, AppendEntriesResponse};
use std::cmp;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use serde_json;
use actix::Actor;
use actix::Handler;
use actix::Context;

const PERSISTENT_STORAGE_FILENAME: &'static str = "raft_persistent_state.json";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ServerStatus {
    Follower,
}

#[derive(Debug)]
pub struct RaftServer {
    status: ServerStatus,
    persistent_state: RwLock<PersistentState>
}

impl RaftServer {
    pub fn new() -> Self {
        let persistent_state = load_persistent_state();

        RaftServer {
            status: ServerStatus::Follower,
            persistent_state: RwLock::new(persistent_state)
        }
    }

    pub fn append_entries(&self, message: AppendEntriesRequest) -> AppendEntriesResponse {
        let term_response;
        {
            let mut persistent_state = self.persistent_state.write().unwrap();

            persistent_state.current_term = cmp::max(persistent_state.current_term, message.term);
            term_response = persistent_state.current_term;

            let mut file = File::create(PERSISTENT_STORAGE_FILENAME).unwrap();
            let json = serde_json::to_string(&*persistent_state).unwrap();
            file.write(json.as_bytes()).unwrap();
        }

        AppendEntriesResponse {
            term: term_response,
            success: true
        }
    }
}

impl Actor for RaftServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("STARTED!");
    }
}

impl Handler<AppendEntriesRequest> for RaftServer {
    type Result = AppendEntriesResponse;

    fn handle(&mut self, msg: AppendEntriesRequest, _ctx: &mut Self::Context) -> Self::Result {
        self.append_entries(msg)
    }
}

fn load_persistent_state() -> PersistentState {
    if Path::new(PERSISTENT_STORAGE_FILENAME).exists() {
        let mut file = File::open(PERSISTENT_STORAGE_FILENAME).unwrap();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).unwrap();
        serde_json::from_slice::<PersistentState>(&bytes).unwrap()
    } else {
        PersistentState {
            current_term: 0,
            voted_for: None,
            log: vec![]
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
