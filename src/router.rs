use crate::requests::Request;
// use anyhow:: Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{ AsyncWriteExt, Error};
/* 
pub trait Router: Send {
    async fn route(&self, request:&mut Request) -> Result<()>;
}
*/

pub trait Router: Send + Sync {
    fn route(&self, request: &mut Request) -> anyhow::Result<Vec<u8>>;
    //  fn route(&self, request:Arc< &mut Request<'_>>) -> Result<()>;

}

/* 

// 新的 trait，不包含 async 方法
pub trait SyncRouter {
    fn route(&self, request: &mut  Request) -> Result<()>;
}

// 修改 Router trait
pub trait Router: SyncRouter {
    async fn async_route(&self, request: &mut  Request) -> Result<()>;
}
*/