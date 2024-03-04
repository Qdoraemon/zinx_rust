use crate::loggers::Loggers;
use crate::MAX_BYTE_LENGTH;

pub struct FrameDecoder {
    index: usize,
    byte_buf: Vec<u8>,
    total: usize,
    res: Vec<Vec<u8>>,
    in_byte_buf: Vec<u8>,
    last_byte_buf: Vec<u8>,
}

// const MAX_FRAME_LENGTH: u32 = 256;
const MAX_FRAME_LENGTH: u32 = MAX_BYTE_LENGTH * 2;
const LENGTH_FIELD_OFFSET: usize = 4;

impl FrameDecoder {
    pub fn new() -> Self {
        FrameDecoder {
            index: 0,
            total: 0,
            byte_buf: vec![0; MAX_FRAME_LENGTH as usize],
            last_byte_buf: vec![0; MAX_FRAME_LENGTH as usize],
            res: Vec::new(),
            in_byte_buf: Vec::new(),
        }
    }
    pub fn decode_form_byte_to_vec(&mut self, data: &[u8]) -> Option<Vec<Vec<u8>>> {

        self.in_byte_buf.extend_from_slice(data);
        loop {
            match self.decode() {
                Ok(buf) => match buf {
                    Some(buf) => {
                        Loggers::new().debug("decode ok@##########￥4 ");
                        self.res.push(buf)
                    }
                    None => {
                        // TODO 这里有数组尽可能不要复制
                        Loggers::new().debug("?????????????? decode error 不够长 ");
                        if self.res.len() > 0 {
                            let tmp_res = self.res.clone();
                            self.res.clear();
                            return Some(tmp_res);
                        } else {
                            return None;
                        }
                    }
                },
                Err(_) => {
                    Loggers::new().debug("decode error 不够长 ");
                    return None;
                }
            };
            Loggers::new().debug(
                format!(
                    "--------------  Decode self.total {} self.res.len():{}",
                    self.total,
                    self.res.len()
                )
                .as_str(),
            );
            if self.total == 0 && self.res.len() > 0 {
                // if self.res.len() > 0 {
                let tmp_res = self.res.clone();
                self.res.clear();
                return Some(tmp_res);
            } else if self.total > 0 {
                continue;
            } else {
                return None;
            }
        }
    }

    fn save_last_byte(&mut self){
        let last_byte_buf_length = self.total - self.index;
        self.last_byte_buf[0 .. last_byte_buf_length].copy_from_slice(self.byte_buf[self.index..self.total].to_vec().as_slice());
        // let temp_buf = self.byte_buf[self.index..self.total].to_vec();
        Loggers::new().debug(format!("---------------????? 1111111 temp_buf :{:?} ,length: {}", self.last_byte_buf,last_byte_buf_length).as_str());
        self.byte_buf.fill(0);
        self.byte_buf[0..last_byte_buf_length].copy_from_slice(&self.last_byte_buf[0..last_byte_buf_length]);
        self.total = last_byte_buf_length;
        self.index = last_byte_buf_length;
        self.last_byte_buf.fill(0);
    }

    pub fn decode(&mut self) -> Result<Option<Vec<u8>>, ()> {
        Loggers::new().debug(format!("00000000000000 self.index : {}", self.index).as_str());
        self.byte_buf[self.index..self.index + self.in_byte_buf.len()]
            .copy_from_slice(self.in_byte_buf.as_slice());
        let data_len = self.in_byte_buf.len();
        self.in_byte_buf.clear();
        self.index = 0;
        self.total += data_len;
        Loggers::new().debug(
            format!(
                "self.byte_buf: {:?} , index:{} ",
                self.byte_buf.clone(),
                self.index
            )
            .as_str(),
        );
        Loggers::new()
            .debug(format!(" self.index +data.len() {} ", self.index + data_len).as_str());

        // 长度都不足够的话不行。
        if self.total < LENGTH_FIELD_OFFSET {
            // TODO 如果有剩余要保留
            self.save_last_byte();
            Loggers::new().debug(format!("====== 9999999999 !@#===================self.index: {},self.total: {}",self.index, self.total).as_str());

            return Ok(None);
        }

        let length: u32 = u32::from_le_bytes(
            self.byte_buf[self.index..self.index + 4]
                .try_into()
                .unwrap(),
        );
        Loggers::new().debug(format!(" length {} , data_len:{} ", length, data_len).as_str());
        let body_length = (length + 4) as usize;

        if self.index + body_length > MAX_FRAME_LENGTH as usize {
            // TODO 如果加起来比那个就是没有全部写入（一般不会出现）
            Loggers::new().debug("!!@######");
            return Ok(None);
        }
        if body_length > self.total {
            self.index = self.total;
            Loggers::new().debug("????????????? ");
            return Ok(None);
        }

        // 还需要判断有没有这么多字符

        // self.index += LENGTH_FIELD_OFFSET as usize;
        let length = (length - 4) as usize;

        let mut write_buf = Vec::new();

        // 需要取走的字节数量
        let byte_buf_length = LENGTH_FIELD_OFFSET * 2 + length;
        write_buf.extend_from_slice(
            self.byte_buf[self.index..self.index + byte_buf_length]
                .try_into()
                .unwrap(),
        );
        
        self.index += byte_buf_length;

        Loggers::new().debug(
            format!(
                "not not not not ????? 1129939394848 index :{} , total:{}",
                self.index, self.total
            )
            .as_str(),
        );
        if self.index < self.total {
            Loggers::new().debug(
                format!(
                    "????? 1129939394848 index :{} , total:{}",
                    self.index, self.total
                )
                .as_str(),
            );
            // TODO 这里以后要改成不需要temp
            self.save_last_byte();
        } else {
            self.byte_buf.fill(0);
            self.index = 0;
            self.total = 0;
            Loggers::new().debug(
                format!(
                    "77777777777777777 ????? 1111111 buf:{:?} index :{} , total:{}, ",
                    self.byte_buf, self.index, self.total
                )
                .as_str(),
            );
        }
        Ok(Some(write_buf))

    }
}
