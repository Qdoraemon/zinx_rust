mod frame_decoder;
mod msg_handle;

use tokio::io::{AsyncReadExt, Error};
use tokio::net::{TcpListener, TcpStream};

use frame_decoder::FrameDecoder;

async fn process(tcp_stream: &mut TcpStream) -> bool {
    let (mut read_half, _write_half) = tcp_stream.split();

    let mut buf = vec![0; 1024];

    //读取消息
    match read_half.read_buf(&mut buf).await {
        Ok(_n) => {
            //转换字符串
            let res = String::from_utf8(buf).unwrap();
            println!("size:{},content:{}", _n, res);
            let peer_addr = tcp_stream.peer_addr().unwrap();
            //通过通道发送
            // tx.send((res,peer_addr)).unwrap();
            return true;
        }
        Err(err) => {
            println!("err : {:?}", err);
            return false;
        }
    }
}

struct Connect {
    tcp_stream: TcpStream,
    frame_decoder : FrameDecoder,
}
impl Connect {
    fn new(stream: TcpStream) -> Self {
        Connect { tcp_stream: stream, frame_decoder: FrameDecoder::new() }
    }

    async fn process(&mut self) -> bool {
        let (mut read_half, _write_half) = self.tcp_stream.split();

        let mut buf = vec![0; 1024];

        //读取消息
        match read_half.read_buf(&mut buf).await {
            Ok(_n) => {
                //转换字符串
                let res = String::from_utf8(buf).unwrap();
                println!("size:{},content:{}", _n, res);
                let peer_addr = self.tcp_stream.peer_addr().unwrap();
                //通过通道发送
                // tx.send((res,peer_addr)).unwrap();
                return true;
            }
            Err(err) => {
                println!("err : {:?}", err);
                return false;
            }
        }
    }

    async fn start(mut self) {
        tokio::spawn(async move {
            loop {
                if let false = self.process().await {
                    println!(   "process error");
                    break;
                }
             
            }
        });
    }
}



// async fn handle_client(mut socket: TcpStream) -> Result<(), Error> {

// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "127.0.0.1:9530";
    // 创建一个TCP listener来监听传入连接
    // let listener = TcpListener::bind(addr).await?;

    let listener =  if let Ok(listener) = TcpListener::bind(addr).await {
        listener
    }else{
        return Ok(());
    };

    /* 
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) =>return Ok(()),
    };
    */
    

    loop {
        let (stream, _) = listener.accept().await?;
        let  conn = Connect::new(stream);
        conn.start().await;
        // tokio::spawn(conn.start());

        // 这里我们创建一个单独的线程来处理TCP连接
        /*
        tokio::spawn(async move {
            conn.process().await;
            // loop{
            // if let false = process(&mut stream).await {
            //     println!("failed to process connection; error = {:?}",stream.peer_addr());
            // }
        // }
        });
        */
    }
}
