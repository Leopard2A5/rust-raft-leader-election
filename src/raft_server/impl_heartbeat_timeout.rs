use actix::Handler;
use request_vote::*;
use heartbeat::HeartbeatTimeout;
use raft_server::RaftServer;

impl Handler<HeartbeatTimeout> for RaftServer {
    type Result = ();

    fn handle(
        &mut self,
        _msg: HeartbeatTimeout,
        _ctx: &mut Self::Context
    ) -> <Self as Handler<HeartbeatTimeout>>::Result {
        use reqwest;
        use std::time::Duration;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        let body = RequestVoteRequest {
            term: self.persistent_state.read().unwrap().current_term,
            candidate_id: self.server_id.clone(),
            last_log_index: 2,
            last_log_term: 3,
        };
        info!("POST call to {}", self.partner_address);
        let res: RequestVoteResponse = client.post(&self.partner_address)
            .json(&body)
            .send().unwrap()
            .json().unwrap();
        info!("{:?}", res);

        info!("TIMEOUT!");
        ()
    }
}
