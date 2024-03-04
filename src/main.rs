mod loggers;
mod msg_handle;
mod proto;
mod ltvdecoder_little;
mod frame_decoder;
mod router;
mod message;
mod requests;
mod connect;



// use anyhow::Ok;
use prost::{self, Message};
use tokio::io::{ AsyncWriteExt, Error};
use tokio::net::TcpListener;

use tokio::task;
use loggers::Loggers;
use proto::cmd::Cmd;
use proto::pb::{CLogin, SLogin};

use requests::Request;
use router::Router;
use connect::Connect;
use std::borrow::BorrowMut;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::msg_handle::MsgHandle;
use tokio::sync::RwLock;



// use uuid::Uuid;

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
    
    fn route(&self, request: &mut Request) -> anyhow::Result<Vec<u8>>{
        // let mut msg_handle = self.tmp_msg_handle.lock().unwrap();
        // let mut msg = new_message(request.get_cmd());
        // msg.set_seq(request.get_seq());
        // msg.set_body(request.get_body());
        // msg_handle.send_msg(msg);
        Loggers::new().debug(
            format!("1222222222222222222222222size:????? ")
                .as_str(),
        );

        // let request = request.clone();
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
       
        // let binding = request.conn.clone();
        // let request =  Arc::clone(& request);
        // let  binding = Arc::clone(& request.conn);
        // binding.clone().send(write_buf);
        // let binding = Arc::clone(&request);
        // let request = request.clone();
        // let s = Request::new(request.conn, request.msg);
        // let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        // request.conn.send(write_buf);
        anyhow::Ok(write_buf.to_vec())
        // let handle = task::spawn_blocking(|| {
        //     // let  binding = Arc::clone(& binding);
        //     // binding.clone().send(write_buf);
        //     // let mut request = request;
        //     request;
        //     // request.conn.send(write_buf).await;
        //     println!("12222222222----------------------------------");
        // });
        // handle.join().unwrap();
       /* 
        tokio::spawn(async move {
            let binding = Arc::clone(& request.conn);
            // binding.lock().await.send(write_buf);
        });
        */
        // msg_handler.lock().await.start();
        // let   (_,  write_half) = binding.tcp_stream.split();
    
            // let   (_, mut write_half) = request.conn.lock().await.tcp_stream.split();
            // write_half.write_buf(&mut write_buf.as_slice()).await;
            // write_half.flush().await;
        
        

        // let s = request.conn.tcp_stream.clone();

        // let   (_, mut write_half) = request.conn.tcp_stream.lock().await.split();
        // let mut lock_result = request.conn.lock().;

        /* 
        let mut lock_result = request.conn.lock().await;
        let (_, mut write_half) = lock_result.tcp_stream.split();
        // let mut lock_result = request.conn.lock().await;
        // let s = tcp_stream.lock().await;
        // let (_, mut write_half) = lock_result.split();
        

        // let ( _ ,mut write_half) =  match &mut lock_result {
        //     Ok( stream) => stream.split(),
        //     Err(e) => {
        //         // Loggers::new().error(format!("?????? {:?}", e).as_str());
        //         return Err(anyhow!("???????"));
        //     }
        // };
        

        write_half.write_buf(&mut write_buf.as_slice()).await;
        write_half.flush().await;
        */
        // Ok(())
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
    let login_box = Box::new(Login::new());
    msg_handler.lock().await.add_api(Cmd::CLogin as u32,login_box).await;
    // msg_handler.lock().await.start();
    // println!(":2133");
    // msg_handler.add_api(Cmd::CLogin as u32,Box::new(Login::new()));

    loop {
        let (stream, _) = listener.accept().await?;
        let conn = Connect::new( stream);
        conn.start(msg_handler.clone()).await;
    }
}


/* 

use tokio::select;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn future1() -> String {
    sleep(Duration::from_secs(1)).await;
    String::from("future1")
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });
    loop {
        select! {
            msg = rx.recv() => {
                if let Some(msg) = msg {
                    println!("received: {}", msg);
                }else{
                    break;
                }
            }
            
        }    
    }
    
}

*/