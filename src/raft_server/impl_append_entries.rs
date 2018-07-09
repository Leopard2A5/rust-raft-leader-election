use actix::Handler;
use actix::AsyncContext;
use append_entries::*;
use raft_server::RaftServer;
use std::cmp;
use std::fs::File;
use serde_json;
use raft_server::PERSISTENT_STORAGE_FILENAME;

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
