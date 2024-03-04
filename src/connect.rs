
use tokio::io::{AsyncReadExt, Error};
use tokio::net::TcpStream;


use std::sync::Arc;
use tokio::sync::Mutex;

use crate::frame_decoder::FrameDecoder;
use crate::msg_handle::MsgHandle;
use crate::loggers::Loggers;
use crate::message::new_message;

use crate::requests::Request;

use crate::MAX_BYTE_LENGTH;

pub struct Connect {
    pub tcp_stream: TcpStream,
    frame_decoder: FrameDecoder,

}


impl Connect {
    pub fn new(stream: TcpStream) -> Self {
    
        Connect {
            tcp_stream: stream,
            frame_decoder: FrameDecoder::new(),
        }
    }
    async fn process(&mut self,msg_handler:Arc<Mutex<MsgHandle>>) -> Result<(), Error> {

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
                   
                    let message = new_message(buf.len(),buf);
                    
                    let mut request = Request::new( &mut *self, message);
                    let mut msg_handler = msg_handler.lock().await;
                    match msg_handler.execute(&mut request).await{
                        Ok(_) => {
                            // TODO: 暂时没有想法
                        },
                        Err(err) => {
                            Loggers::new().debug(format!("!!! execute error : {:?}", err).as_str());
                            // return Err(err.into());
                        }
                    };
                  
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
    
}