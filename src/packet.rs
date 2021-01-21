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
                let bit = match HammingPacket::get_bit_at_pos(&self, i) {
                    Ok(val) => val,
                    Err(_) => panic!("overflow error occurred when calculating packet error pos!"),
                };
                if bit {
                    pos ^= i;
                }
            }
            pos
        }
    }
}
