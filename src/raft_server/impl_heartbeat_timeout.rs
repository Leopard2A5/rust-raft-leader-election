use actix::Handler;
use request_vote::*;
use heartbeat::HeartbeatTimeout;
use raft_server::*;
use raft_server::http_req::post;

impl Handler<HeartbeatTimeout> for RaftServer {
    type Result = ();

    fn handle(
        &mut self,
        _msg: HeartbeatTimeout,
        _ctx: &mut Self::Context
    ) -> <Self as Handler<HeartbeatTimeout>>::Result {
        self.status = ServerStatus::Candidate;
        self.persistent_state.write().unwrap().current_term += 1;

        let body = RequestVoteRequest {
            term: self.persistent_state.read().unwrap().current_term,
            candidate_id: self.server_id.clone(),
            last_log_index: 2,
            last_log_term: 3,
        };
        info!("POST call to {}", self.partner_address);
        let res = post(&self.partner_address, &body);
        info!("{:?}", res);

        info!("TIMEOUT!");
        ()
    }
}
