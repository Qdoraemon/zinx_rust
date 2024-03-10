use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use core::time;
use std::error;
use std::sync::Arc;
use std::thread::sleep;
use tokio::sync::Mutex;

use crate::connects::frame_decoder::FrameDecoder;
use crate::connects::message::new_message;
use crate::connects::msg_handle::MsgHandle;
use crate::loggers::Loggers;

use crate::connects::requests::Request;

use crate::MAX_BYTE_LENGTH;

pub struct Connect {
    // pub tcp_stream: TcpStream,
    frame_decoder: FrameDecoder,
    pub tcp_stream: Arc<Mutex<TcpStream>>,
    // read_half: Box<ReadHalf<'a>>
    // read_half : OwnedReadHalf,
    // write_half: OwnedWriteHalf,
}

impl Connect {
    pub async fn new(tcp_stream: Arc<Mutex<TcpStream>>) -> Self {
        Connect {
            tcp_stream: tcp_stream,
            frame_decoder: FrameDecoder::new(),
            // read_half: read_half,
            // write_half: write_half,
        }
    }

    pub async fn send(&mut self, msg: Vec<u8>) {
        println!("2221000000000000000000000000");
        let tcp_stream = self.tcp_stream.clone();

        println!("3221000000000000000000000000");
        let mut tcp_stream = tcp_stream.lock().await;
        println!("4221000000000000000000000000");
        let (_, mut write_half) = tcp_stream.split();
        match write_half.write_buf(&mut msg.as_slice()).await {
            Ok(n) => {
                Loggers::new().debug(format!("call success write size:{}", n).as_str());
            }
            Err(e) => {
                Loggers::new().warn(format!("call error :{}", e).as_str());
                // ret/urn Err(e.into());
                return ;
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
        println!("1000000000000000000000000");
        //     }

        // }).await;
    }

    async fn process(&mut self, msg_handler: Arc<Mutex<MsgHandle>>) -> Result<Vec<Vec<u8>>, Error> {
        // let tcp_streamss = Arc::clone(&tcp_stream);
        println!("111111111111111111");
        let tcp_stream = self.tcp_stream.clone();
        println!("2222222222222222222");
        let mut tcp_stream = tcp_stream.lock().await;
        // self.read_half = Arc::new (Mutex::new( read_half));
        // self.write_half = &write_half;
        println!("333333333333333333333");
        // tcp_stream.set_ttl(3);
        let ( read_half, _) = tcp_stream.split();

        let mut buf = Vec::with_capacity(MAX_BYTE_LENGTH as usize);
        // let mut buf = [0; MAX_BYTE_LENGTH as usize];
        // let mut buf = Vec::new();
        println!("44444444444444444444444444");

        //读取消息
        match read_half.try_read_buf(&mut buf) {
            Ok(0) => {
                println!("123456=======================================");
                return Err( Error::from( std::io::ErrorKind::ConnectionRefused));
                // return Ok(vec![])

            },
            Ok(n) => {
                println!("5555555555555555555555555555");
                if n <= 0 {
                    println!("2222----------------------{:?}", &buf);
                    return Ok(vec![]);
                }
                println!("!!!!!!!!! 2222----------------------n:{}, {:?}", n, &buf);
                println!("3333");
                // let buf = &buf[0..n];
                let bufs = match self.frame_decoder.decode_form_byte_to_vec(&buf) {
                    Some(bufs) => bufs,
                    None => return Ok(vec![]),
                };

                Loggers::new()
                    .debug(format!("========================== bufs:{}", bufs.len()).as_str());
                /*
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
                            Loggers::new().debug("process ok");
                            return Ok(());
                        },
                        Err(err) => {
                            Loggers::new().debug(format!("!!! execute error : {:?}", err).as_str());
                            // return Err(err.into());
                        }
                    };

                }
                */

                Ok(bufs)
            }
            Err(err) => {
                Loggers::new().debug(format!("!!! err : {:?}", err).as_str());
                Err(err)
            }
        }
    }
    
    pub async fn close(&mut self)->Result<(),Error>{
        let mut tcp_stream = self.tcp_stream.lock().await;
        match tcp_stream.shutdown().await{
            Ok(_) => {
                Loggers::new().debug("close ok");
                return Ok(());
            },
            Err(err) => {
                Loggers::new().debug(format!("!!! close error : {:?}", err).as_str());
                return Err(err);
            }
        }
    }
    
    /* 
    async fn start_heart_beat(&mut self) {
        tokio::spawn(async move {
            loop {
                println!("1111");
                sleep(time::Duration::from_secs(1))
            }
        });
    }
    */
    pub async fn call_msg_handle(&mut self,bufs :Vec<Vec<u8>>, msg_handler: Arc<Mutex<MsgHandle>>)  {
        if bufs.len() <= 0 {
            return 
        }
        for buf in bufs {
            //转换字符串
            Loggers::new().debug(
                format!("########################!000000000000000000000000000000 buf:{:?} ",buf).as_str()
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
                    Loggers::new()
                        .debug(format!("!!! execute error : {:?}", err).as_str());
                    // return Err(err.into());
                }
            };
        }
        
    }

    pub async fn start(mut self, msg_handler: Arc<Mutex<MsgHandle>>) {
        
        let  heart_beat: HeartBeat = HeartBeat::new(self.tcp_stream.clone());

        
        tokio::spawn(async move {
            // select! {
            // }

            loop {
                match self.process(Arc::clone(&msg_handler)).await {
                    Ok(bufs) => {
                        Loggers::new().debug("process ok11");
                        self.call_msg_handle(bufs, Arc::clone(&msg_handler)).await;
                       
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        Loggers::new().debug(format!("again ====").as_str());
                        continue;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::ConnectionRefused => {
                        Loggers::new().debug(format!(" tcp close !!!!====").as_str());
                        return;
                    }
                    Err(err) => {
                        
                        Loggers::new().warn(format!("--- !!! err : {:?}", err.to_string()).as_str());
                        return ;
                    }
                }

                println!("22333");
                // sleep(time::Duration::from_secs(5))
            }
        });
        
        // let s = Arc::new(Mutex::new(&mut self.tcp_stream));
        heart_beat.start().await;
        // s.await.unwrap();
    }
}

struct HeartBeat {
    conn: Arc<Mutex<TcpStream>>
}
impl HeartBeat {
    pub fn new(conn: Arc<Mutex<TcpStream>>) -> Self {
        Self {conn:conn}
    }

    pub async fn start(self) {
        // conn.lock().await.close().await;

        tokio::spawn(async move {
            println!("1000999999999999999");
            
            loop {
                // println!("1111");
                let mut tcp_stream = self.conn.lock().await;
                // let mut tcp_stream = self.tcp_stream.lock().await;
                
                sleep(time::Duration::from_secs(5));
                /* 
                match tcp_stream.shutdown().await{
                    Ok(_) => {
                        Loggers::new().debug("close ok");
                        // return Ok(());
                        return;
                    },
                    Err(err) => {
                        Loggers::new().debug(format!("!!! close error : {:?}", err).as_str());
                        // return Err(err);
                    }
                }
                */
                // conn.lock().await.close().await;
                // s.shutdown().await;
                println!("finish close");
                // return;
            }
        });
    }
}
