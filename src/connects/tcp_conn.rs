use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::TcpStream;

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::connects::frame_decoder::FrameDecoder;
use crate::connects::message::new_message;
use crate::connects::msg_handle::MsgHandle;
use crate::loggers::Loggers;

use crate::connects::requests::Request;

use crate::MAX_BYTE_LENGTH;
use crate::connects::heart_beat::{HeartBeat, HeartBeatTime};
use core::time;

pub struct Connect {

    frame_decoder: FrameDecoder,
    pub tcp_stream: Arc<Mutex<TcpStream>>,

}

impl Connect {
    pub async fn new(tcp_stream: Arc<Mutex<TcpStream>>) -> Self {
        Connect {
            tcp_stream: tcp_stream,
            frame_decoder: FrameDecoder::new(),

        }
    }

    pub async fn send(&mut self, msg: Vec<u8>) {
        let tcp_stream = self.tcp_stream.clone();

        Loggers::new().debug("3221000000000000000000000000");
        let mut tcp_stream = tcp_stream.lock().await;
        Loggers::new().debug("4221000000000000000000000000");
        let (_, mut write_half) = tcp_stream.split();
        match write_half.write_buf(&mut msg.as_slice()).await {
            Ok(n) => {
                Loggers::new().debug(format!("call success write size:{}", n).as_str());
            }
            Err(e) => {
                Loggers::new().warn(format!("1111call error :{}", e.kind()).as_str());
                return;
            }
        }

        match write_half.flush().await {
            Ok(_) => {
                Loggers::new().debug(format!("call success flush").as_str());
            }
            Err(e) => {
                Loggers::new().warn(format!("call error :{}", e).as_str());
                // return Err(e.into());
                return;
            }
        }
        Loggers::new().debug("1000000000000000000000000");
    }

    async fn process(
        &mut self,
        heart_beat_time: Arc<Mutex<HeartBeatTime>>,
    ) -> Result<Vec<Vec<u8>>, Error> {
       
        let tcp_stream = self.tcp_stream.clone();
       
        let mut tcp_stream = tcp_stream.lock().await;
        
        let ( read_half, _) = tcp_stream.split();

        let mut buf = Vec::with_capacity(MAX_BYTE_LENGTH as usize);
        // let mut buf = [0; MAX_BYTE_LENGTH as usize];
        // let mut buf = Vec::new();

        //读取消息
        match read_half.try_read_buf(&mut buf) {
            Ok(0) => {
                Loggers::new().debug("123456=======================================");
                return Err(Error::from(std::io::ErrorKind::ConnectionRefused));
                // return Ok(vec![])
            }
            Ok(n) => {
                heart_beat_time.lock().await.update_time();
                Loggers::new().debug("5555555555555555555555555555");
                if n <= 0 {
                    // println!("2222----------------------{:?}", &buf);
                    return Ok(vec![]);
                }
                Loggers::new()
                    .debug(format!("!!!!!!!!! 2222----------------------n:{}, {:?}", n, &buf).as_str());
                // let buf = &buf[0..n];
                let bufs = match self.frame_decoder.decode_form_byte_to_vec(&buf) {
                    Some(bufs) => bufs,
                    None => return Ok(vec![]),
                };

                Loggers::new()
                    .debug(format!("========================== bufs:{}", bufs.len()).as_str());

                Ok(bufs)
            }
            Err(err) => {
                // TODO : 暂时注释掉
                // Loggers::new().debug(format!("!!! err : {:?}", err).as_str());
                Err(err)
            }
        }
    }
    /* 
    pub async fn close(&mut self) -> Result<(), Error> {
        let mut tcp_stream = self.tcp_stream.lock().await;
        match tcp_stream.shutdown().await {
            Ok(_) => {
                Loggers::new().debug("close ok");
                return Ok(());
            }
            Err(err) => {
                Loggers::new().debug(format!("!!! close error : {:?}", err).as_str());
                return Err(err);
            }
        }
    }
    */

    pub async fn call_msg_handle(
        &mut self,
        bufs: Vec<Vec<u8>>,
        msg_handler: Arc<Mutex<MsgHandle>>,
    ) {
        if bufs.len() <= 0 {
            return;
        }
        for buf in bufs {
            //转换字符串
            Loggers::new().debug(
                format!(
                    "########################!000000000000000000000000000000 buf:{:?} ",
                    buf
                )
                .as_str(),
            );

            let message = new_message(buf.len(), buf);

            let mut request = Request::new(&mut *self, message);
            let mut msg_handler = msg_handler.lock().await;
            match msg_handler.execute(&mut request).await {
                Ok(_) => {
                    // TODO: 暂时没有想法
                    Loggers::new().debug("process ok");
                    // return Ok(());
                }
                Err(err) => {
                    Loggers::new().debug(format!("!!! execute error : {:?}", err).as_str());
                    // return Err(err.into());
                }
            };
        }
    }

    pub async fn start(mut self, msg_handler: Arc<Mutex<MsgHandle>>,test_num :u32) {
        let heart_beat_time = Arc::new(Mutex::new(HeartBeatTime::new(30,test_num)));
        let heart_beat = HeartBeat::new(self.tcp_stream.clone(), heart_beat_time.clone(), 10);

        tokio::spawn(async move {
            // select! {
            // }

            loop {
                // sleep(time::Duration::from_secs(1));
                match self.process(Arc::clone(&heart_beat_time)).await {
                    Ok(bufs) => {
                        Loggers::new().debug("process ok11");
                        self.call_msg_handle(bufs, Arc::clone(&msg_handler)).await;
                        continue;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // Loggers::new().debug(format!("again ====").as_str());
                        tokio::time::sleep(time::Duration::from_millis(10)).await;
                        continue;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                        Loggers::new().warn(format!(" tcp close !!!!====").as_str());
                        return;
                    }
                    Err(err) => {
                        Loggers::new()
                            .warn(format!("tcp close !!!!==== --- !!! err : {:?}", err.to_string()).as_str());
                        return;
                    }
                }

                Loggers::new().debug("connect finish");
               
            }
        });
        
        heart_beat.start(|i|{
            Loggers::new().debug(format!("after timeout {}",i ).as_str());
        }).await;

        
    }
}

