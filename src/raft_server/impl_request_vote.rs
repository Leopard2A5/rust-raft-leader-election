use actix::Handler;
use request_vote::*;
use ::raft_server::RaftServer;


impl Handler<RequestVoteRequest> for RaftServer {
    type Result = RequestVoteResponse;

    fn handle(
        &mut self,
        _msg: RequestVoteRequest,
        _ctx: &mut Self::Context
    ) -> <Self as Handler<RequestVoteRequest>>::Result {
        unimplemented!()
    }
}
