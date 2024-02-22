pub struct FrameDecoder {
    index: usize,
    byte_buf: Vec<u8>,
    total: usize,
    res: Vec<Vec<u8>>,
    in_byte_buf: Vec<u8>,
}
use crate::MAX_BYTE_LENGTH;

const MAX_FRAME_LENGTH: u32 = MAX_BYTE_LENGTH * 2;
const LENGTH_FIELD_OFFSET: usize = 4;



impl FrameDecoder {
    pub fn new() -> Self {
        // FrameDecoder{index:0,byte_buf:Vec::with_capacity(MAX_FRAME_LENGTH as usize)}
        FrameDecoder {
            index: 0,
            total: 0,
            byte_buf: vec![0; MAX_FRAME_LENGTH as usize],
            res: Vec::new(),
            in_byte_buf: Vec::new(),
        }
    }
    pub fn Decode(&mut self, data: &[u8]) -> Option<Vec<Vec<u8>>> {
        // let mut res = Vec::new();
        self.in_byte_buf.extend_from_slice(data);
        loop {
            // self.decode(data)
            match self.decode() {
                Ok(buf) => match buf {
                    Some(buf) => {
                        println!("decode ok@##########￥4 ");
                        self.res.push(buf)
                    }
                    None => {
                        // TODO 这里有数组尽可能不要复制
                        println!("?????????????? decode error 不够长 ");
                        if self.res.len() > 0 {
                            let tmp_res = self.res.clone();
                            self.res.clear();
                            return Some(tmp_res);
                        }else{
                            return None;
                        }
                       
                    }
                },
                Err(_) => {
                    // println!("decode error:{}", err);
                    println!("decode error 不够长 ");
                    return None;
                }
            };
            println!(
                "--------------  Decode self.total {} self.res.len():{}",
                self.total,
                self.res.len()
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

    pub fn decode(&mut self) -> Result<Option<Vec<u8>>, ()> {
        println!("00000000000000 self.index : {}", self.index);
        self.byte_buf[self.index..self.index + self.in_byte_buf.len()]
            .copy_from_slice(self.in_byte_buf.as_slice());
        let data_len = self.in_byte_buf.len();
        self.in_byte_buf.clear();
        self.index = 0;
        self.total += data_len;
        println!(
            "self.byte_buf: {:?} , index:{} ",
            self.byte_buf.clone(),
            self.index
        );
        println!(" self.index +data.len() {} ", self.index + data_len);

        // 长度都不足够的话不行。
        if self.total < LENGTH_FIELD_OFFSET {
            println!("!@#===================");
            return Ok(None);
        }

        let length: u32 = u32::from_le_bytes(
            self.byte_buf[self.index..self.index + 4]
                .try_into()
                .unwrap(),
        );
        println!(" length {} , data_len:{} ", length, data_len);
        let body_length = (length + 4) as usize;

        if self.index + body_length > MAX_FRAME_LENGTH as usize {
            // TODO 如果加起来比那个就是没有全部写入（一般不会出现）
            println!("!!@######");
            return Ok(None);
        }
        if body_length > self.total {
            self.index = self.total;
            println!("????????????? ");
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
        // self.index += LENGTH_FIELD_OFFSET ;
        // write_buf.extend_from_slice(self.byte_buf[self.index ..self.index+4].try_into().unwrap());
        // // 偏移4个字节
        // self.index += LENGTH_FIELD_OFFSET;
        // write_buf.extend_from_slice(self.byte_buf[self.index ..self.index+length].try_into().unwrap());
        self.index += byte_buf_length;

        println!(
            "not not not not ????? 1129939394848 index :{} , total:{}",
            self.index, self.total
        );
        if self.index < self.total {
            println!(
                "????? 1129939394848 index :{} , total:{}",
                self.index, self.total
            );
            // TODO 这里以后要改成需要temp
            let temp_buf = self.byte_buf[self.index..self.total].to_vec();
            println!("????? 1111111 temp_buf :{:?} ", temp_buf);
            self.byte_buf.fill(0);
            self.byte_buf[0..temp_buf.len()].copy_from_slice(&temp_buf);
            self.total = temp_buf.len();
            self.index = temp_buf.len();
        } else {
            self.byte_buf.fill(0);
            self.index = 0;
            self.total = 0;
            println!(
                "77777777777777777 ????? 1111111 buf:{:?} index :{} , total:{}, ",
                self.byte_buf, self.index, self.total
            );
        }
        Ok(Some(write_buf))

        // if self.byte_buf.len() < LENGTH_FIELD_OFFSET as usize {
        //     return Ok(None);
        // }

        // let length = u32::from_be_bytes([self.byte_buf[0],self.byte_buf[1],self.byte_buf[2],self.byte_buf[3]]);
        // if self.byte_buf.len() < length as usize{
        //     return Ok(None);
        // }
    }
}
