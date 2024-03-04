use crate::requests::Request;


pub trait Router: Send + Sync {
    fn route(&self, request: &mut Request) -> anyhow::Result<Vec<u8>>;

}
