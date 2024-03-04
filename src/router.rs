use crate::requests::Request;

pub enum RouterResult {
    Send(Vec<u8>),
    OK,

}

pub trait Router: Send + Sync {
    fn route(&self, request: &mut Request) -> anyhow::Result<RouterResult>;

}
