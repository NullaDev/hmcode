pub const HEADER_BYTES: usize = 5;
pub const DATA_BYTES: usize = 4096;
pub const DATA_BYTES_VALID: usize = 4091;

pub mod hamming_packet {
    use processor::{bits2byte, byte2bits};

    use crate::byte_lib::processor;
    use std::convert::TryInto;

    pub struct HammingPacket {
        index: i16,
        size: i16,
        frag_flag: u8,
        data_bytes: [u8; super::DATA_BYTES],
    }

    impl HammingPacket {
        pub fn get_bit_at_pos(&self, pos: usize) -> Result<bool, String> {
            if pos > super::DATA_BYTES * 8 {
                Err(format!(
                    "The position provided is {}, and out of range!",
                    pos
                ))
            } else {
                let index = pos / 8;
                let target_bits = processor::byte2bits(self.data_bytes[index]);
                Ok(target_bits[pos % 8])
            }
        }

        pub fn set_bit_at_pos(&mut self, pos: usize, bit: bool) -> Result<String, String> {
            if pos > super::DATA_BYTES {
                Err(String::from("The position provided is out of range!"))
            } else {
                let index: usize = pos / 8;
                let target_byte = self.data_bytes[index];
                let mut target_bits = processor::byte2bits(target_byte);
                target_bits[pos % 8] = bit;
                self.data_bytes[index] = processor::bits2byte(&target_bits);
                Ok(String::from("The target bit has set."))
            }
        }

        pub fn to_raw_bytes(&self) -> [u8; super::DATA_BYTES] {
            let raw_bytes = self.data_bytes.clone();
            raw_bytes
        }

        pub fn is_final(&self) -> bool {
            self.frag_flag == 0
        }

        pub fn get_index(&self) -> i16 {
            let index = self.index.clone();
            index
        }

        pub fn info(&self) -> String {
            format!(
                "Packet index: {}, size: {}, fragflag: {}. ",
                self.index, self.size, self.frag_flag
            )
        }

        pub fn calc_err_pos(&self) -> usize {
            let mut pos: usize = 0;
            for i in 0..(8 * super::DATA_BYTES) {
                let bit = self.get_bit_at_pos(i).expect("Overflow error occurred.");
                if bit {
                    pos ^= i;
                }
            }
            pos
        }

        pub fn self_correct(&mut self) -> Result<String, String> {
            let pos = self.calc_err_pos();
            if pos == 0 {
                println!("No error found.");
                Ok(String::from("No error found."))
            } else {
                let mut flag = 0;
                for i in 0..(8 * super::DATA_BYTES) {
                    let bit = self.get_bit_at_pos(i).expect("Overflow error occurred.");
                    if bit {
                        flag += 1;
                    }
                }
                if flag % 2 == 0 {
                    Err(String::from(
                        "Two or more error found, cannot self correct.",
                    ))
                } else {
                    let correct_bit = match self.get_bit_at_pos(pos) {
                        Ok(val) => !val,
                        Err(info) => panic!(info),
                    };
                    let success_message = match self.set_bit_at_pos(pos, correct_bit) {
                        Ok(_) => format!(
                            "Packet {} found wrong bit at pos {}, corrected",
                            self.index, pos
                        ),
                        Err(info) => panic!(info),
                    };
                    println!("{}", success_message);
                    Ok(success_message)
                }
            }
        }

        pub fn to_real_bytes(&mut self) -> Result<Vec<u8>, String> {
            match self.self_correct() {
                Ok(_) => {
                    let mut bit_buffer = Vec::new();
                    let data_buffer = self.data_bytes.to_vec();
                    for i in data_buffer.iter() {
                        bit_buffer.append(&mut byte2bits(*i).to_vec());
                    }
                    for i in (0..15).rev() {
                        bit_buffer.remove(1 << i);
                    }
                    bit_buffer.remove(0);
                    let mut real_bytes: Vec<u8> = Vec::new();
                    for i in super::HEADER_BYTES..(super::HEADER_BYTES + (self.size as usize)) {
                        let byte = processor::bits2byte(&bit_buffer[8 * i..8 * (i + 1)]);
                        real_bytes.push(byte);
                    }
                    Ok(real_bytes)
                }
                Err(info) => Err(info),
            }
        }

        pub fn from_packed_bytes(data: &Vec<u8>) -> Result<HammingPacket, String> {
            if data.len() != super::DATA_BYTES {
                Err(String::from("Invalid data size."))
            } else {
                let bytes_buffer = data.clone();
                let mut bits_buffer: Vec<bool> = Vec::new();
                for byte in bytes_buffer.iter() {
                    bits_buffer.append(&mut byte2bits(*byte).to_vec());
                }
                for i in (0..15).rev() {
                    bits_buffer.remove(1 << i);
                }
                bits_buffer.remove(0);

                let mut header_bytes: Vec<u8> = Vec::new();
                for i in 0..super::HEADER_BYTES {
                    header_bytes.push(bits2byte(&bits_buffer[8 * i..8 * (i + 1)]));
                }
                let _index = processor::bytes2short(&header_bytes[0..2]);
                let _size = processor::bytes2short(&header_bytes[2..4]);
                let _fragflag = header_bytes[4];
                Ok(HammingPacket {
                    index: _index,
                    size: _size,
                    frag_flag: _fragflag,
                    data_bytes: bytes_buffer.try_into()
                    .unwrap_or_else(|v: Vec<u8>| panic!(
                        "Error occurred when tyring to convert a vector to an array! The length of the vector is {}, hope this may be useful.",
                        v.len())),
                })
            }
        }

        pub fn from_bytes(
            index: i16,
            fragflag: u8,
            data: &Vec<u8>,
        ) -> Result<HammingPacket, String> {
            if data.len() > super::DATA_BYTES_VALID {
                Err(String::from("Too many bytes in a packet!"))
            } else if fragflag != 0 && fragflag != 1 {
                Err(format!("Invalid fragflag value: {}!", fragflag))
            } else {
                // 创建头字节
                println!("Start creating header bytes.");
                let _index: i16 = index;
                let _size: i16 = data.len() as i16;
                let _fragflag: u8 = fragflag;
                let mut data_buffer: Vec<bool> = Vec::new();
                data_buffer
                    .append(&mut processor::byte2bits(processor::short2bytes(_index)[0]).to_vec());
                data_buffer
                    .append(&mut processor::byte2bits(processor::short2bytes(_index)[1]).to_vec());
                data_buffer
                    .append(&mut processor::byte2bits(processor::short2bytes(_size)[0]).to_vec());
                data_buffer
                    .append(&mut processor::byte2bits(processor::short2bytes(_size)[1]).to_vec());
                data_buffer.append(&mut processor::byte2bits(_fragflag).to_vec());
                for byte in data.iter() {
                    data_buffer.append(&mut processor::byte2bits(*byte).to_vec());
                }

                // 填补空余空间
                println!("Start filling up the rest space.");
                let mut fill_up_size = super::DATA_BYTES_VALID as i16 - _size;
                while fill_up_size >= 0 {
                    data_buffer.append(
                        &mut [false, false, false, false, false, false, false, false].to_vec(),
                    );
                    fill_up_size -= 1;
                }

                println!("the whole vector's size is {}", data_buffer.len());

                // 插入汉明码
                println!("Start inserting the Hamming code.");
                data_buffer.insert(0, false);
                for i in 0..15 {
                    data_buffer.insert((1 << i) as usize, false);
                }

                let mut flag: i32 = 0;
                for i in 0..(8 * 4096) {
                    if *data_buffer
                        .get(i)
                        .expect("Error when getting element in data buffer.")
                    {
                        flag ^= i as i32;
                    }
                }
                for i in 0..15 {
                    if (flag >> i) & 0x1 == 1 {
                        if let Some(elem) = data_buffer.get_mut(1 << i) {
                            *elem = true;
                        }
                    }
                }

                let mut _flag = 0;
                for bit in data_buffer.iter() {
                    if *bit {
                        _flag += 1;
                    }
                }
                if _flag % 2 == 1 {
                    if let Some(elem) = data_buffer.get_mut(0) {
                        *elem = true;
                    }
                }

                let mut data_bytes: Vec<u8> = Vec::new();
                for i in 0..super::DATA_BYTES {
                    data_bytes.push(processor::bits2byte(&data_buffer[8 * i..8 * (i + 1)]));
                }

                // 返还创建的包
                println!("Return the packet.");
                Ok(HammingPacket {
                    index: _index,
                    size: _size,
                    frag_flag: _fragflag,
                    data_bytes: data_bytes.try_into()
                    .unwrap_or_else(|v: Vec<u8>| panic!(
                        "Error occurred when tyring to convert a vector to an array! The length of the vector is {}, hope this may be useful.",
                        v.len())),
                    })
            }
        }
    }
}

pub mod packet_handle {
    use super::hamming_packet::HammingPacket;

    pub fn handle_single_packet(data: &Vec<u8>) -> HammingPacket {
        match HammingPacket::from_bytes(0, 0, data) {
            Ok(pak) => pak,
            Err(info) => panic!(info),
        }
    }

    pub fn restore_single_packet(packet: &mut HammingPacket) -> Vec<u8> {
        match packet.to_real_bytes() {
            Ok(val) => val,
            Err(info) => panic!(info),
        }
    }

    pub fn handle_multi_packet(data: &Vec<u8>) -> Vec<HammingPacket> {
        let count = data.len() / super::DATA_BYTES_VALID + 1;
        let mut packets: Vec<HammingPacket> = Vec::new();
        for i in 0..count {
            let _size = if i == count - 1 {
                data.len() % super::DATA_BYTES_VALID
            } else {
                super::DATA_BYTES_VALID
            };
            let mut packet_bytes: Vec<u8> = Vec::new();
            for j in 0..packet_bytes.len() {
                packet_bytes.push(data[i * super::DATA_BYTES_VALID + j]);
            }
            let index: i16 = i as i16;
            let fragflag: u8 = if i == count - 1 { 1 } else { 0 };
            packets[index as usize] =
                match HammingPacket::from_bytes(index, fragflag, &packet_bytes) {
                    Ok(val) => val,
                    Err(info) => panic!(info),
                }
        }
        packets
    }

    pub fn handle_existed_pak(data: &Vec<u8>) -> HammingPacket {
        match HammingPacket::from_packed_bytes(data) {
            Ok(val) => val,
            Err(info) => panic!(info),
        }
    }
}
