use crate:: connects::ltvdecoder_little::LtvLittleDecoder;
use crate::connects::requests::Request;
use crate::connects::router::{Router,RouterResult};
use anyhow:: Result;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use crate::loggers::Loggers;
pub struct MsgHandle {

    apis: Mutex<HashMap<u32, Box<dyn Router + Send + Sync>>>,
}

impl MsgHandle {
    pub fn new() -> MsgHandle {
        MsgHandle {
            apis: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_api(&mut self, msg_id: u32, api: Box<dyn Router + Sync + 'static + Send>) {
        if self.apis.lock().await.contains_key(&msg_id) {
            return;
        }
        self.apis.lock().await.insert(msg_id, api);
    }

    pub async fn execute(&mut self, request: &mut Request<'_>) -> Result<()> {
        let mut decode = LtvLittleDecoder::new();
        if let Err(e) = decode.intercept(&mut request.msg) {
            Loggers::new().warn(format!("execute error :{}",e).as_str());
            return Err(e);
        }
        match self.call(request).await{
            Ok(()) => {
                Loggers::new().debug(format!("execute success ").as_str());
                return Ok(());
            }
            Err(e) => {
                Loggers::new().warn(format!("execute error :{}",e).as_str());
                return Err(e);
            }
        }
    }

    pub async fn call(&mut self, request: &mut Request<'_>) -> Result<()> {
        if let Some(api) = self.apis.lock().await.get(&request.msg.get_id()) {
            match api.route(request) {
                Ok(result) =>  match result{
                    RouterResult::Send(message) =>{
                        println!("12223!!!!!!!!!!!!!!!!");
                        let tcp_stream = request.conn.send( message).await;
                        /* 
                        let tcp_stream = request.conn.tcp_stream.clone();
                        let mut tcp_stream = tcp_stream.lock().await;
                        let (_, mut write_half) = tcp_stream.split();
                        // println!("222333333333333333333--------------------");
                        match write_half.write_buf(&mut message.as_slice()).await {
                            Ok(n) => {
                                Loggers::new().debug(format!("call success write size:{}",n).as_str());
                            }
                            Err(e) => {
                                Loggers::new().warn(format!("call error :{}",e).as_str());
                                return Err(e.into());
                            }
                        }
    
                        match write_half.flush().await{
                            Ok(_) => {
                                Loggers::new().debug(format!("call success flush").as_str());
                            }
                            Err(e) => {
                                Loggers::new().warn(format!("call error :{}",e).as_str());
                                return Err(e.into());
                            }
                        }
                        */
                        return Ok(());
                    },
                    RouterResult::OK =>{
                        Loggers::new().debug(format!("call ok.... ").as_str());
                        return Ok(());
                    },

                }
                Err(er) => {
                    println!("-------!!!!!!?>>>>>>>>>>>>>>>>>>>{}", er);
                }
            }
            return Ok(());
        }
        Ok(())
    }


}
