
use anyhow::{anyhow, Result};

use crate::connects::message::Message;
pub struct LtvLittleDecoder{
    length: u32 ,//L
	tag    :u32, //T
	value : Vec<u8>, //V
}
const LTV_HEADER_SIZE:u32 = 8; //表示TLV空包长度

impl LtvLittleDecoder{
    pub fn new() -> Self {
        LtvLittleDecoder {
            length: 0,
            tag: 0,
            value: Vec::new(),
        }
    }
    
    pub fn decode(&mut self, buf: Vec<u8>) {
        // Implement decoding logic here
        let length = &buf[0..4];
        self.length = u32::from_le_bytes(length.try_into().unwrap());
        let types = &buf[4..8];
        self.tag = u32::from_le_bytes(types.try_into().unwrap());
        // 8..length as usize + 4
        self.value =  buf[8..self.length as usize + 4].to_vec();
        // Loggers::new().debug(format!("00000000!!! ---------------?????  ,length: {:?},tags :{},value:{:?}", self.length,self.tag,self.value).as_str());
        
        
        
    }
    pub fn intercept(&mut self,message: &mut Message) ->Result<()>{
        let buf = &message.data;
        if buf.len() < LTV_HEADER_SIZE as usize {
            return Err(anyhow!("An error occurred"));
            
        }
        self.decode(buf.to_vec());
        message.set_data(self.value.clone());
        message.set_data_len(self.value.len() as u32);
        message.set_id(self.tag);
        Ok(())

    }
}