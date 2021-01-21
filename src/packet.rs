pub mod packet {
    use crate::byte_lib::processor;
    use std::convert::TryInto;

    pub const HEADER_BYTES: usize = 5;
    pub const DATA_BYTES: usize = 4096;
    pub const DATA_BYTES_VALID: usize = 4089;

    pub struct HammingPacket {
        index: i16,
        size: i16,
        frag_flag: u8,
        data_bytes: [u8; DATA_BYTES],
    }

    impl HammingPacket {
        pub fn get_bit_at_pos(&self, pos: usize) -> Result<bool, String> {
            if pos > DATA_BYTES {
                Err(String::from("The position provided is out of range!"))
            } else {
                let index = pos / 8;
                let target_bits = processor::byte2bits(self.data_bytes[index]);
                Ok(target_bits[pos % 8])
            }
        }

        pub fn set_bit_at_pos(&mut self, pos: usize, bit: bool) -> Result<String, String> {
            if pos > DATA_BYTES {
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

        pub fn to_raw_bytes(&self) -> [u8; DATA_BYTES] {
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
            for i in 0..(8 * DATA_BYTES) {
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
                Ok(String::from("No error found."))
            } else {
                let mut flag = 0;
                for i in 0..(8 * DATA_BYTES) {
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
                    Ok(success_message)
                }
            }
        }

        pub fn to_real_bytes(&mut self) -> Result<Vec<u8>, String> {
            match self.self_correct() {
                Ok(_) => {
                    let mut bit_buffer = Vec::new();
                    for i in 0..8 * DATA_BYTES {
                        bit_buffer.push(self.get_bit_at_pos(i).expect("Overflowed!"));
                    }
                    for i in (0..15).rev() {
                        bit_buffer.remove(1 << i as usize);
                    }
                    bit_buffer.remove(0);
                    let mut real_bytes: Vec<u8> = Vec::new();
                    for i in HEADER_BYTES..HEADER_BYTES + self.size as usize {
                        let byte = processor::bits2byte(&bit_buffer[8 * i..8 * i + 8]);
                        real_bytes.push(byte);
                    }
                    Ok(real_bytes)
                }
                Err(info) => Err(info),
            }
        }

        // TODO
        pub fn from(index: i16, fragflag: u8, data: &[u8]) -> Result<HammingPacket, String> {
            if data.len() > DATA_BYTES_VALID {
                Err(String::from("Too many bytes in a packet!"))
            } else if fragflag != 0 || fragflag != 1 {
                Err(String::from("Invalid fragflag value!"))
            } else {
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
                for byte in data.iter() {
                    data_buffer.append(&mut processor::byte2bits(*byte).to_vec());
                }

                let fill_up_size = DATA_BYTES_VALID as i16 - _size;
                while fill_up_size > 0 {
                    data_buffer.append(
                        &mut [false, false, false, false, false, false, false, false].to_vec(),
                    );
                }

                data_buffer.insert(0, false);
                for i in 0..15 {
                    data_buffer.insert((1 << i) as usize, false);
                }

                let mut flag: i32 = 0;
                for i in 0..8 * DATA_BYTES {
                    if data_buffer[i] {
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
                for i in 0..DATA_BYTES {
                    data_bytes.push(processor::bits2byte(&data_buffer[8 * i..8 * (i + 1)]));
                }

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
