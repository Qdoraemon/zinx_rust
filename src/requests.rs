use crate::Connect;// Request 请求
use crate::message::Message;// 请求数据
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
use std::rc::Rc;

pub struct Request <'a> {
	pub conn    :&'a mut Connect ,    // the connection which has been established with the client(已经和客户端建立好的链接)
	pub msg     :Message,        // the request data sent by the client(客户端请求的数据)
    // pub conn :  Arc< &'a mut Connect>,
    // pub conn :  Arc< Mutex< &'a mut Connect>>,

}

impl <'a> Request<'a> {
    // pub fn new(conn:   Arc< &'a mut Connect>, msg:Message) ->  Request {
    pub fn new(conn:   &'a mut Connect, msg:Message) ->  Request {
        // let conn =  Arc::clone(&conn);
        Request{  conn, msg}
    }
    pub fn bing_router(&self)  {
        
    }
}
