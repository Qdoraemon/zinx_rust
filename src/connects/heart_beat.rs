use crate::loggers::Loggers;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::Instant;

use crate::proto::cmd::Cmd;
use core::time;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub struct HeartBeatTime {
    max_time: u64,
    last_time: Instant,
    // TODO : 以后会删除掉，用于测试。
    test_num: u32,
    is_close : bool,
}
impl HeartBeatTime {
    pub fn new(max_time: u64, test_num: u32) -> Self {
        Loggers::new().debug(
            format!(
                "test_num:{:?}   ----  timeout : {:?}",
                test_num,
                Instant::now()
            )
            .as_str(),
        );

        Self {
            max_time: max_time,
            last_time: Instant::now(),
            test_num: test_num,
            is_close : false,
        }
    }
    pub fn set_close(&mut self,close :bool){
        self.is_close = close;
    }

    pub fn is_time_out(&self) -> bool {
        let time_duration = Instant::now().duration_since(self.last_time).as_secs();

        Loggers::new().info(
            format!(
                "test_num:{:?} ===== timeout : {:?}",
                self.test_num, time_duration
            )
            .as_str(),
        );

        if time_duration > self.max_time || self.is_close == true{
            return true;
        }
        return false;
    }
    pub fn update_time(&mut self) {
        self.last_time = Instant::now();
    }
}

pub struct HeartBeat {
    tcp: Arc<Mutex<TcpStream>>,
    heart_beat_time: Arc<Mutex<HeartBeatTime>>,
    heart_time: u64,
}
impl HeartBeat {
    pub fn new(
        tcp: Arc<Mutex<TcpStream>>,
        heart_beat_time: Arc<Mutex<HeartBeatTime>>,
        heart_time: u64,
    ) -> Self {
        Self {
            tcp: tcp,
            heart_beat_time: heart_beat_time,
            heart_time: heart_time,
        }
    }

    pub async fn start<F: Fn(u32) + Send + 'static>(self, f: F) {
        // conn.lock().await.close().await;

        tokio::spawn(async move {
            Loggers::new().debug("1000999999999999999");
            loop {
                tokio::time::sleep(time::Duration::from_secs(self.heart_time)).await;

                let mut tcp_stream = self.tcp.lock().await;
                if self.heart_beat_time.lock().await.is_time_out() {
                    // TODO :暂时没有做错误处理
                    match tcp_stream.shutdown().await {
                        Ok(_) => {
                            Loggers::new().warn("time out close ok");
                        }
                        Err(err) => {
                            Loggers::new().warn(format!("!!! close error : {:?}", err).as_str());
                        }
                    }
                    f(1);
                    return;
                }

                let data = Vec::new();
                let login = Cmd::GHeart as u32;
                let types: [u8; 4] = login.to_le_bytes();
                let length: [u8; 4] = ((data.len() + types.len()) as u32).to_le_bytes();
                let mut write_buf = Vec::new();
                write_buf.extend_from_slice(&length);
                write_buf.extend_from_slice(&types);
                write_buf.extend_from_slice(&data);

                let _ = tcp_stream.write(write_buf.as_slice()).await;
                let _ = tcp_stream.flush().await;
            }
        });
    }
}
