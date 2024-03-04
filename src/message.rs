pub struct Message {
    // 消息长度
    pub data_len: u32,
    // 消息内容
    pub data: Vec<u8>,
    // 原始数据
    pub raw_data: Vec<u8>,
    pub id:      u32, // ID of the message
}
impl Message {
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_data_len(&mut self, len: u32) {
        self.data_len = len;
    }
    pub fn get_data_len(&self) -> u32 {
        self.data_len
    }
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
    pub fn set_raw_data(&mut self, data: Vec<u8>) {
        self.raw_data = data;
    }
    pub fn get_raw_data(&self) -> &Vec<u8> {
        &self.raw_data
    }

}

pub fn new_message(len:usize, data: Vec<u8>) ->Message {
	Message{
		data_len: len as u32,
		data:    data.clone(),
		raw_data: data.clone(),
		id:       0,
	}
}