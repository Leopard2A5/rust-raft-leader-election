
use reqwest;
use std::time::Duration;
use actix::Message;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub(crate) fn post<'a, 'b, M>(
    endpoint: &'a str,
    msg: &M
) -> M::Result
where M: Message,
      M: Serialize,
      M::Result: DeserializeOwned
{
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();

    let ret: <M as Message>::Result = client.post(endpoint)
        .json(msg)
        .send().unwrap()
        .json().unwrap();
    ret
}
