use crate::Connect;// Request 请求
use crate::connects::message::Message;// 请求数据

pub struct Request <'a> {
	pub conn    :&'a mut Connect ,    // the connection which has been established with the client(已经和客户端建立好的链接)
	pub msg     :Message,        // the request data sent by the client(客户端请求的数据)

}

impl <'a> Request<'a> {
    // pub fn new(conn:   Arc< &'a mut Connect>, msg:Message) ->  Request {
    pub fn new(conn:   &'a mut Connect, msg:Message) ->  Request {
        // let conn =  Arc::clone(&conn);
        Request{  conn, msg}
    }
    pub fn _bing_router(&self)  {
        
    }
}
