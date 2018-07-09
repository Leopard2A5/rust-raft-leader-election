use ::Term;
use heartbeat::*;
use std::sync::RwLock;
use std::fs::File;
use std::path::Path;
use serde_json;
use actix::{
    Actor,
    Context,
    SpawnHandle,
    AsyncContext,
};
use rand::prelude::*;

mod impl_request_vote;
mod impl_append_entries;
mod impl_heartbeat_timeout;

const PERSISTENT_STORAGE_FILENAME: &'static str = "raft_persistent_state.json";
const HEARTBEAT_TIMEOUT_MIN: u64 = 3000;
const HEARTBEAT_TIMEOUT_MAX: u64 = 6000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ServerStatus {
    Follower,
}

#[derive(Debug)]
pub struct RaftServer {
    status: ServerStatus,
    persistent_state: RwLock<PersistentState>,
    heartbeat_timeout_handle: Option<SpawnHandle>,
    server_id: String,
    partner_address: String
}

impl RaftServer {
    pub fn new(
        server_id: String,
        partner_address: String
    ) -> Self {
        let persistent_state = load_persistent_state();

        RaftServer {
            status: ServerStatus::Follower,
            persistent_state: RwLock::new(persistent_state),
            heartbeat_timeout_handle: None,
            server_id,
            partner_address
        }
    }

    fn schedule_timeout(&mut self, ctx: &mut Context<Self>) {
        use std::time::Duration;

        let timeout_millis = thread_rng().gen_range(HEARTBEAT_TIMEOUT_MIN, HEARTBEAT_TIMEOUT_MAX);
        let timeout = Duration::from_millis(timeout_millis);
        info!("schedule timeout in {} millis", timeout_millis);
        self.heartbeat_timeout_handle = Some(
            ctx.notify_later(HeartbeatTimeout, timeout)
        );
    }
}

impl Actor for RaftServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Raft server started");
        self.schedule_timeout(ctx);
    }
}

fn load_persistent_state() -> PersistentState {
    use std::io::Read;

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
    current_term: Term,
    voted_for: Option<String>,
    log: Vec<LogEntry>
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum LogEntry {

}
