use crate::requests::{self, Request};
use crate::router::Router;
use crate::ltvdecoder_little::LtvLittleDecoder;
use std::collections::HashMap;
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::select;


use std::sync::mpsc;

pub struct MsgHandle {
    // protocol_builder: ProtocolBuilder,
    // apis:HashMap<u32, Box< dyn Router>>,
    // apis: Arc<HashMap<u32, Box<dyn Router  + 'static + Send>>>,
    apis:Mutex< HashMap<u32,  Box<dyn Router +Send + Sync>>>,
    pub send_channel : mpsc::Sender<u8>,
    pub recv_channel : Arc<Mutex< mpsc::Receiver<u8>>>,
    // pub recv_channel : mpsc::Receiver<Vec<u8>>>,
    
}






impl MsgHandle {

    pub fn new() -> MsgHandle {
        let (send_channel, recv_channel) = mpsc::channel();
        
        MsgHandle { 
            // protocol_builder: ProtocolBuilder::new(),
            // apis: HashMap::new(),
            apis: Mutex::new(HashMap::new()),
            send_channel: send_channel,
            recv_channel: Arc::new(Mutex::new( recv_channel)),
        }
    }

    pub  async fn add_api(&mut self,msg_id: u32, api: Box< dyn Router +Sync + 'static + Send>) {
        println!("asdsadasd=============================");
        if self.apis.lock().await.contains_key(&msg_id) {
            return;
        }
        // let arc_api = Arc::new(Mutex::new(api));
        self.apis.lock().await.insert(msg_id, api);
    }
    /* 
    pub  fn start(&mut self,request:&mut Request<'_>) {
        
        let recv_channel = Arc::clone(&self.recv_channel);
        
        tokio::spawn(async move {
            recv_channel.clone().lock().await.recv();
            
            // loop {
            //     select! {
            //         msg = recv_channel.lock().await.recv() => {
            //             match msg {
            //                 Some(data) => {
            //                     println!("received: {}", data);
            //                 }
            //                 None => {
            //                     // 处理通道关闭或其他终止条件
            //                     println!("通道关闭或其他终止条件");
            //                     break;
            //                 }
            //             }
            //         }
            //     }
            // }
        });
        
    }
    */

    pub async  fn execute(&mut self,request:  &mut Request<'_>) -> Result<()>{
        let mut decode = LtvLittleDecoder::new();
        if let Err(e) = decode.intercept( &mut request.msg){
            return Err(e);
        }
        // let (s ,s1 ) = request.conn.clone().tcp_stream.split();
        self.call(request).await;
        println!("-------!!!!!!");
        Ok(())
        // request.msg.Data
        
        // let (tx,rx)  = mpsc::channel();
        // self.protocol_builder.execute(request);
        // request.msg.data
    }

    
    pub async   fn call(&mut self,request:&mut  Request<'_> ){
        
        if let Some(api) = self.apis.lock().await.get(&request.msg.get_id()) {
            // let api = api.clone();
            // let request = Arc::new(Mutex::new(request));
            // let requests = Arc::new(request);
            match api.route(request) {
                Ok(message ) => {
                    // request.conn (message).await;
                    let (_,mut s ) = request.conn.tcp_stream.split();
                    s.write_buf(&mut message.as_slice()).await;
                    s.flush().await;
                    println!("111sssss 123912983890123");

                },
                Err(er) => {
                    println!("-------!!!!!!?>>>>>>>>>>>>>>>>>>>{}",er);
                }
            }
                
            

            
            // 在这里对 Mutex 进行加锁
            // let guard = api.lock().unwrap();
            // guard.route(request);
        }
        // self.apis.get(&request.msg.id).unwrap().route(request);
    }

    /* 
    pub fn add_api(&mut self,msg_id: u32, api:Box<dyn Router>) {
        
        if self.apis.contains_key(&msg_id) {
            return;
        }
        // let arc_api = Arc::new(Mutex::new(api));
        self.apis.insert(msg_id, api);
        // self.apis.entry(msg_id).or_insert(api);
        
        /* 
        if !self.apis.lock().await.contains_key(&msg_id) {
            self.apis.lock().await.insert(msg_id, api);
        }
        */
    }
    */
    /* 
    pub async fn add_api(&mut self, msg_id: u32, api: Box<dyn Router>) {
        if self.apis.contains_key(&msg_id) {
            return;
        }
        // let arc_api = Arc::new(Mutex::new(api));
        self.apis.insert(msg_id, api);
        // self.apis.entry(msg_id).or_insert(api);
        
        /* 
        if !self.apis.lock().await.contains_key(&msg_id) {
            self.apis.lock().await.insert(msg_id, api);
        }
        */
    }
    */
    



    
    
}