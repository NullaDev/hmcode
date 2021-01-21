pub mod packet {
    use crate::byte_lib::processor;

    pub const HEADER_BYTES: usize = 5;
    pub const DATA_BYTES: usize = 4096;
    pub const DATA_BYTES_VALID: usize = 4089;

    pub struct HammingPacket {
        index: i16,
        size: i16,
        frag_flag: u8,
        data_bytes: Vec<u8>,
    }

    impl HammingPacket {
        pub fn get_bit_at_pos(&self, pos: usize) -> Result<bool, String> {
            if pos > DATA_BYTES {
                Err(String::from("the position provided is out of range!"))
            } else {
                let index = pos / 8;
                let target_bits = processor::byte2bits(self.data_bytes[index]);
                Ok(target_bits[pos % 8])
            }
        }

        pub fn set_bit_at_pos(&mut self, pos: usize, bit: bool) -> Result<String, String> {
            if pos > DATA_BYTES {
                Err(String::from("the position provided is out of range!"))
            } else {
                let index: usize = pos / 8;
                let target_byte = self.data_bytes[index];
                let mut target_bits = processor::byte2bits(target_byte);
                target_bits[pos % 8] = bit;
                self.data_bytes[index] = processor::bits2byte(&target_bits);
                Ok(String::from("the target bit has set."))
            }
        }

        pub fn to_raw_bytes(&self) -> Vec<u8> {
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
                "packet index: {}, size: {}, fragflag: {}. ",
                self.index, self.size, self.frag_flag
            )
        }

        pub fn calc_err_pos(&self) -> usize {
            let mut pos: usize = 0;
            for i in 0..(8 * DATA_BYTES) {
                let bit = self.get_bit_at_pos(i).expect("overflow error occurred.");
                if bit {
                    pos ^= i;
                }
            }
            pos
        }

        pub fn self_correct(&mut self) -> Result<String, String> {
            let pos = self.calc_err_pos();
            if pos == 0 {
                Ok(String::from("no error found."))
            } else {
                let mut flag = 0;
                for i in 0..(8 * DATA_BYTES) {
                    let bit = self.get_bit_at_pos(i).expect("overflow error occurred.");
                    if bit {
                        flag += 1;
                    }
                }
                if flag % 2 == 0 {
                    Err(String::from(
                        "two or more error found, cannot self correct.",
                    ))
                } else {
                    let correct_bit = match self.get_bit_at_pos(pos) {
                        Ok(val) => !val,
                        Err(info) => panic!(info),
                    };
                    let success_message = match self.set_bit_at_pos(pos, correct_bit) {
                        Ok(_) => format!(
                            "packet {} found wrong bit at pos {}, corrected",
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
                        let byte = processor::bits2byte(&bit_buffer[8 * i..8 * i + 8].to_vec());
                        real_bytes.push(byte);
                    }
                    Ok(real_bytes)
                }
                Err(info) => Err(info),
            }
        }

        // TODO
        //pub fn from(index: i16, size: i16, fragflag: u8, data: &Vec<u8>) -> HammingPacket {}
    }
}
