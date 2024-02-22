mod frame_decoder;
mod msg_handle;
mod proto;

use std::io::Read;

use prost::{self, Message};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::{TcpListener, TcpStream};

use frame_decoder::FrameDecoder;

use proto::cmd::Cmd;
use proto::pb::{CLogin, SLogin};

// use uuid::Uuid;

const MAX_BYTE_LENGTH :u32 = 128;
// const MAX_BYTE_LENGTH: u32 = 10;

struct Connect {
    tcp_stream: TcpStream,
    frame_decoder: FrameDecoder,
}
impl Connect {
    fn new(stream: TcpStream) -> Self {
        Connect {
            tcp_stream: stream,
            frame_decoder: FrameDecoder::new(),
        }
    }

    async fn process(&mut self) -> bool {
        let (mut read_half, mut write_half) = self.tcp_stream.split();

        let mut buf = Vec::with_capacity(MAX_BYTE_LENGTH as usize);

        //读取消息
        match read_half.read_buf(&mut buf).await {
            Ok(n) => {
                /*
                let buf = match self.frame_decoder.decode(&buf){
                    Ok(buf) => match buf{
                        Some(buf) => buf,
                        None => {
                            println!("?????????????? decode error 不够长 {} ",n);
                            return true;
                        }
                    },
                    Err(_) => {
                        // println!("decode error:{}", err);
                        println!("decode error 不2@33够长 {} ",n);
                        return false;
                    }
                };
                */
                let bufs = match self.frame_decoder.Decode(&buf) {
                    Some(bufs) => bufs,
                    None => return true,
                };
                println!("========================== bufs:{}", bufs.len());
                for buf in bufs {
                    //转换字符串
                    // let res = String::from_utf8(buf).unwrap();
                    println!(
                        "########################!000000000000000000000000000000 buf:{:?} size:{}",
                        buf, n
                    );
                    // let peer_addr = self.tcp_stream.peer_addr().unwrap();
                    // let s = &buf[0..n];
                    let length = &buf[0..4];
                    let length = u32::from_ne_bytes(length.try_into().unwrap());
                    let types = &buf[4..8];
                    let types = u32::from_ne_bytes(types.try_into().unwrap());

                    // 内容
                    let body_a = &buf[8..8 + 2];
                    let login_type = u16::from_le_bytes(body_a.try_into().unwrap());

                    let res = String::from_utf8(buf[8 + 2..length as usize + 4].to_vec()).unwrap();

                    println!("asdasdasd!!!! ");
                    // println!("size:{},content:{}", n, res);
                    // buf.truncate(n)
                    // buf.truncate(n);
                    // protobuf::parse_from_bytes::<Request>(&buf[8+2..length as usize + 4]).unwrap();

                    println!("size:{:?}, n:{}, length:{},type:{}", buf, n, length, types);
                    println!("login_type:{},content :{}", login_type, res);

                    let s = CLogin::decode(&buf[8..length as usize + 4]).unwrap();
                    println!("????? login_type:{},content :{}", s.r#type, s.steam_id);
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

                    println!("???????? data:{:?}", data);
                    println!("???????? write_buf:{:?}", write_buf);

                    write_half
                        .write_buf(&mut write_buf.as_slice())
                        .await
                        .unwrap();
                    write_half.flush().await.unwrap();
                    // write_half.write(&mut write_buf).await.unwrap();
                    // write_half.flush().await.unwrap();

                    // write_buf.extend_from_slice(&types);
                    // write_half.write_buf(&mut write_buf);

                    //通过通道发送
                    // tx.send((res,peer_addr)).unwrap();
                }

                return true;
            }
            Err(err) => {
                println!("!!! err : {:?}", err);
                return false;
            }
        }
    }

    async fn start(mut self) {
        tokio::spawn(async move {
            loop {
                // self.process().await;
                if let false = self.process().await {
                    println!("process error");
                    break;
                }
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    /*
        参考资料
        let mut  s = vec![0u8; 20];
        let mut  buf = vec![1u8,2,3,4,5,6,7,8,9,10];
        println!("s:{:?}",s);
        let buf_slice = &buf[0..4];
        println!("buf_slice ---- :{:?}", buf_slice.len());
        s[2..2 + buf_slice.len()].copy_from_slice(&buf_slice);
        // s[0..].extend_from_slice(&buf);
        println!("s:{:?}",s);
        s[10..13].fill(1);
        println!("s:{:?}",s);

        Ok(())
    */

    let addr = "127.0.0.1:9530";
    // 创建一个TCP listener来监听传入连接

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(_) => return Ok(()),
    };

    loop {
        let (stream, _) = listener.accept().await?;
        let conn = Connect::new(stream);
        conn.start().await;
    }
}
