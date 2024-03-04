
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::TcpStream;

use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;


use crate::frame_decoder::FrameDecoder;
use crate::msg_handle::MsgHandle;
use crate::loggers::Loggers;
use crate::message::new_message;
use crate::proto::cmd::Cmd;
use crate::requests::Request;
use crate::Login;
use tokio::sync::RwLock;
use std::sync::mpsc;


use crate::MAX_BYTE_LENGTH;



pub struct Connect {
    pub tcp_stream: TcpStream,
    // pub tcp_stream:Arc<Mutex<TcpStream>>,
    frame_decoder: FrameDecoder,
    // msg_handler: Arc<Mutex<MsgHandle>>,
    // pub send_channel : mpsc::Sender<u8>,
    // pub recv_channel : Arc<Mutex< mpsc::Receiver<u8>>>,

    
    // tmp_msg_handle : Arc<Mutex<MsgHandle>>,
}


impl Connect {
    pub fn new(stream: TcpStream) -> Self {
        // let (send_channel, recv_channel) = mpsc::channel();
        Connect {
            // tcp_stream: Arc::new(Mutex::new(stream)),
            tcp_stream: stream,
            frame_decoder: FrameDecoder::new(),
            // msg_handler: msg_handler,
            // send_channel: send_channel,
            // recv_channel: Arc::new(Mutex::new( recv_channel)),
     
            // tmp_msg_handle:  Arc::new(Mutex::new(MsgHandle::new())),
        }
    }
    async fn process(&mut self,msg_handler:Arc<Mutex<MsgHandle>>) -> Result<(), Error> {
        /* 
        let ss = Arc::clone(&self.tcp_stream);
        let mut s = ss.lock().await;
        let (mut read_half, _) = s.split();
        */
        let (mut read_half, _) =   self.tcp_stream.split();
        

    
        let mut buf = Vec::with_capacity(MAX_BYTE_LENGTH as usize);

        //读取消息
        match read_half.read_buf(&mut buf).await {
            Ok(n) => {
                let bufs = match self.frame_decoder.decode_form_byte_to_vec(&buf) {
                    Some(bufs) => bufs,
                    None => return Ok(()),
                };

                Loggers::new()
                    .debug(format!("========================== bufs:{}", bufs.len()).as_str());

                for buf in bufs {

                    //转换字符串
                    Loggers::new().debug(
                        format!("########################!000000000000000000000000000000 buf:{:?} size:{}",
                        buf, n).as_str()
                    );
                    // let mut msg_handler = self.msg_handler.lock().await;
                    
                   
                    let message = new_message(buf.len(),buf);
                    // let arc_connect = Arc::new(Mutex::new(&self));
                    // let s = Rc::new(self);
                    // let conn_rc = Arc::new( &mut *self);
                    
                    let mut s = Request::new( &mut *self, message);
                    // conn_rc.clone().borrow_mut().tmp_msg_handle.execute(s);
                    // let  aaa = conn_rc.clone();
                    // aaa.tmp_msg_handle.execute(s);
                    let mut msg_handler = msg_handler.lock().await;
                    msg_handler.execute(&mut s).await;
                  
                }
                
                Ok(())
              
            }
            Err(err) => {
                Loggers::new().debug(format!("!!! err : {:?}", err).as_str());
                Err(err)

            }
        }
        
    }

    pub async fn start(mut self,msg_handler:Arc<Mutex<MsgHandle>>) {

        tokio::spawn( async move {
            loop {
                match self.process( Arc::clone(& msg_handler)).await {
                    Ok(_) => {
                        Loggers::new().debug("process ok");
                    }
                    Err(err) => {
                        Loggers::new().warn(format!("--- !!! err : {:?}", err).as_str());
                        return ;
                    }
                }
            }
        });
    }

    pub  fn send(&mut self, message: Vec<u8>) {
        
        // let recv_channel = Arc::clone(&self.recv_channel);
        // let mut  s = request.conn.clone();
        // let tcp_stream = self.tcp_stream;
        // let ss = Arc::clone(&self.tcp_stream);
        // ss.lock().await.write(src);
        // println!("233333!!!");
        // ss.clone().lock()?.try_write(buf);
        let (_,mut s ) = self.tcp_stream.split();
        s.write(message.as_slice());
    
        // ss.lock().await.write_buf(&mut message.as_slice()).await;
        // ss.lock().await.flush().await;
        println!("00-------- 233333");
        
        // tokio::spawn(async move {
        //     let (_,mut s ) = self.tcp_stream.split();
        //     // println!("!!!! {:?}",message);
        //     // ss.lock().await.write_buf(&mut message.as_slice()).await;
        //     println!("23333311!!!!!!!!!!!!!!!");
        //     s.write_buf(&mut message.as_slice()).await;
        //     // let (_ ,mut s1) = self.tcp_stream.split();
        //     // // recv_channel.clone().lock().await.recv();
        //     // s1.write_buf(&mut message.as_slice()).await;
            
        // });
        
        
    }


/* 
    async fn process(&mut self) -> Result<(),Error> {

        

        // let (mut read_half, _) = self.tcp_stream.split();
        let mut lock_result = self.tcp_stream.lock().await;
        let (mut read_half, _) = lock_result.split();
        // let (mut read_half, mut write_half) =  match &mut lock_result {
        //     Ok( stream) => match stream{
        //         stream => stream.split(),
        //     },
        //     Err(e) => {
        //         return Err(Error::new(
        //                 ErrorKind::Other, 
        //                 format!("?????? lock stream error:{:?}",e).as_str())); 
        //     }
        // };

    
        let mut buf = Vec::with_capacity(MAX_BYTE_LENGTH as usize);

        //读取消息
        match read_half.read_buf(&mut buf).await {
            Ok(n) => {
                let bufs = match self.frame_decoder.decode_form_byte_to_vec(&buf) {
                    Some(bufs) => bufs,
                    None => return Ok(()),
                };

                Loggers::new()
                    .debug(format!("========================== bufs:{}", bufs.len()).as_str());

                for buf in bufs {

                    //转换字符串
                    Loggers::new().debug(
                        format!("########################!000000000000000000000000000000 buf:{:?} size:{}",
                        buf, n).as_str()
                    );
                    let mut msg_handler = self.msg_handler.lock().await;
                    msg_handler.add_api(Cmd::CLogin as u32,Box::new(Login::new()));
                    /* 
                    let message = new_message(buf.len(),buf);
                    let mut s = Request::new(&self, message);
                    self.tmp_msg_handle.lock().unwrap().execute(&mut s);
                    */
                    let message = new_message(buf.len(),buf);
                    let mut s = Request::new(&self, message);
                    // let s = Rc::new(RefCell::new(s));
                    // msg_handler.execute(&mut s);
                    /* 
                    let conn_rc = Rc::new(Box::new(&mut *self));
                    let s = Rc :: new(Request::new( Rc::clone(&conn_rc), message));
                    // conn_rc.clone().borrow_mut().tmp_msg_handle.execute(s);
                    let  aaa = conn_rc.clone();
                    aaa.tmp_msg_handle.execute(s);
                    */
                    // self.tmp_msg_handle.borrow_mut().execute(Rc ::clone(&s));
                    // let peer_addr = self.tcp_stream.peer_addr().unwrap();
                    // let s = &buf[0..n];
                    /* 
                    let length = &buf[0..4];
                    let length = u32::from_ne_bytes(length.try_into().unwrap());
                    let types = &buf[4..8];
                    let types = u32::from_ne_bytes(types.try_into().unwrap());

                    // 内容
                    let body_a = &buf[8..8 + 2];
                    let login_type = u16::from_le_bytes(body_a.try_into().unwrap());

                    let res = String::from_utf8(buf[8 + 2..length as usize + 4].to_vec()).unwrap();

                    Loggers::new().debug("asdasdasd!!!! ");

                    Loggers::new().debug(
                        format!("size:{:?}, n:{}, length:{},type:{}", buf, n, length, types)
                            .as_str(),
                    );
                    Loggers::new()
                        .debug(format!("login_type:{},content :{}", login_type, res).as_str());

                    let s = CLogin::decode(&buf[8..length as usize + 4]).unwrap();
                    Loggers::new().debug(
                        format!("????? login_type:{},content :{}", s.r#type, s.steam_id).as_str(),
                    );
                    let mut write_buf = Vec::new();

                    let login = Cmd::SLogin as u32;

                    let types: [u8; 4] = login.to_le_bytes();

                    let s_login = SLogin {
                        r#type: 0,
                        res: 0,
                        data: None,
                        token: res,
                    };
                    let mut data = Vec::new();
                    let _ = s_login.encode(&mut data);
                    let length: [u8; 4] = ((data.len() + types.len()) as u32).to_le_bytes();
                    write_buf.extend_from_slice(&length);
                    write_buf.extend_from_slice(&types);
                    write_buf.extend_from_slice(&data);

                    Loggers::new().debug(format!("???????? data:{:?}", data).as_str());
                    Loggers::new().debug(format!("???????? write_buf:{:?}", write_buf).as_str());

                    write_half
                        .write_buf(&mut write_buf.as_slice())
                        .await
                        .unwrap();
                    write_half.flush().await.unwrap();
                    */
                }
                
                Ok(())
              
            }
            Err(err) => {
                Loggers::new().debug(format!("!!! err : {:?}", err).as_str());
                Err(err)

            }
        }
    }
    */

    
}