use actix::Message;

pub struct HeartbeatTimeout;

impl Message for HeartbeatTimeout {
    type Result = ();
}
