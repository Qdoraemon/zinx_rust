mod loggers;

mod proto;

mod connects;



use prost::{self, Message};
use tokio::io:: Error;
use tokio::net::TcpListener;
use loggers::Loggers;
use proto::cmd::Cmd;
use proto::pb::{CLogin, SLogin};

use connects::requests::Request;
use connects::router::{Router,RouterResult};
use connects::tcp_conn::Connect;
use crate::connects::msg_handle::MsgHandle;
use std::sync::Arc;
use tokio::sync::Mutex;



const MAX_BYTE_LENGTH: u32 = 1024;
// const MAX_BYTE_LENGTH: u32 = 10;


struct Login {

}
impl Login {
    fn new() -> Self {
        Self{}
    }
}
impl Router for Login {
    
    fn route(&self, request: &mut Request) -> anyhow::Result<RouterResult>{

        Loggers::new().debug(
            format!("1222222222222222222222222size:????? ")
                .as_str(),
        );

     
        let res = CLogin::decode(request.msg.data.as_slice()).unwrap();

        let s_login = SLogin {
            r#type: 0,
            res: 0,
            data: None,
            token: res.steam_id,
        };
        let mut data = Vec::new();
        let _ = s_login.encode(&mut data);
        let login = Cmd::SLogin as u32;
        let types: [u8; 4] = login.to_le_bytes();
        let length: [u8; 4] = ((data.len() + types.len()) as u32).to_le_bytes();
        let mut write_buf = Vec::new();
        write_buf.extend_from_slice(&length);
        write_buf.extend_from_slice(&types);
        write_buf.extend_from_slice(&data);

        Loggers::new().debug(format!("???????? data:{:?}", data).as_str());
        Loggers::new().debug(format!("???????? write_buf:{:?}", write_buf).as_str());
       
        let result = RouterResult::Send(write_buf.to_vec());
        anyhow::Ok(result)
    }
}

struct HeartBeat {}
impl HeartBeat {
    fn new() -> Self {
        Self{}
    }
}

impl Router for HeartBeat {
    fn route(&self,request:&mut  Request) -> anyhow::Result<RouterResult> {
        Loggers::new().debug(format!("heard").as_str());
        anyhow::Ok(RouterResult::OK)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    Loggers::init();

    let addr = "127.0.0.1:9530";
    // 创建一个TCP listener来监听传入连接

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(_) => return Ok(()),
    };
    // TODO :暂时没有用不上
    let msg_handler = Arc::new(Mutex::new(MsgHandle::new()));
    msg_handler.lock().await.add_api(Cmd::GHeart as u32,Box::new(HeartBeat::new())).await;
    msg_handler.lock().await.add_api(Cmd::CLogin as u32,Box::new(Login::new())).await;
   
    loop {
        let (stream, _) = listener.accept().await?;
        let stream = Arc::new(Mutex::new(stream));
        let mut conn = Connect::new( stream).await;
        
        conn.start(msg_handler.clone()).await;
    }
}