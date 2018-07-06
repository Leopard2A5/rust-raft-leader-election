use ::Term;
use append_entries::*;
use heartbeat::*;
use request_vote::*;
use std::sync::RwLock;
use std::cmp;
use std::fs::File;
use std::path::Path;
use serde_json;
use actix::{
    Actor,
    Handler,
    Context,
    SpawnHandle,
    AsyncContext,
};

const PERSISTENT_STORAGE_FILENAME: &'static str = "raft_persistent_state.json";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ServerStatus {
    Follower,
}

#[derive(Debug)]
pub struct RaftServer {
    status: ServerStatus,
    persistent_state: RwLock<PersistentState>,
    heartbeat_timeout_handle: Option<SpawnHandle>
}

impl RaftServer {
    pub fn new() -> Self {
        let persistent_state = load_persistent_state();

        RaftServer {
            status: ServerStatus::Follower,
            persistent_state: RwLock::new(persistent_state),
            heartbeat_timeout_handle: None
        }
    }

    fn schedule_timeout(&mut self, ctx: &mut Context<Self>) {
        use std::time::Duration;

        info!("schedule timeout!");
        self.heartbeat_timeout_handle = Some(
            ctx.notify_later(HeartbeatTimeout, Duration::new(5, 0))
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

impl Handler<AppendEntriesRequest> for RaftServer {
    type Result = AppendEntriesResponse;

    fn handle(
        &mut self,
        msg: AppendEntriesRequest,
        ctx: &mut Self::Context
    ) -> Self::Result {
        use std::io::Write;

        let term_response;
        {
            let mut persistent_state = self.persistent_state.write().unwrap();

            persistent_state.current_term = cmp::max(persistent_state.current_term, msg.term);
            term_response = persistent_state.current_term;

            let mut file = File::create(PERSISTENT_STORAGE_FILENAME).unwrap();
            let json = serde_json::to_string(&*persistent_state).unwrap();
            file.write(json.as_bytes()).unwrap();
        }

        if let Some(handle) = self.heartbeat_timeout_handle {
            info!("cancel timeout!");
            ctx.cancel_future(handle);
        }

        self.schedule_timeout(ctx);

        AppendEntriesResponse {
            term: term_response,
            success: true
        }
    }
}

impl Handler<HeartbeatTimeout> for RaftServer {
    type Result = ();

    fn handle(
        &mut self,
        _msg: HeartbeatTimeout,
        _ctx: &mut Self::Context
    ) -> <Self as Handler<HeartbeatTimeout>>::Result {
        info!("TIMEOUT!");
        ()
    }
}

impl Handler<RequestVoteRequest> for RaftServer {
    type Result = RequestVoteResponse;

    fn handle(
        &mut self,
        msg: RequestVoteRequest,
        ctx: &mut Self::Context
    ) -> <Self as Handler<RequestVoteRequest>>::Result {
        unimplemented!()
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
