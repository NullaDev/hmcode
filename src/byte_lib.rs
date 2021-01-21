pub mod processor {
    pub fn byte2bits(byte: u8) -> Vec<bool> {
        let mut bits: Vec<bool> = Vec::new();
        for pos in 0..8 {
            bits.push(((byte >> (7 - pos)) & 0x1) != 0);
        }
        bits
    }

    #[test]
    fn test_byte2bits() {
        assert_eq!(
            byte2bits(7),
            vec![false, false, false, false, false, true, true, true]
        );
    }

    pub fn bits2byte(bits: &Vec<bool>) -> u8 {
        let mut byte: u8 = 0;
        for pos in 0..8 {
            if (*bits)[pos] {
                byte += 1 << pos;
            }
        }
        byte
    }

    #[test]
    fn test_bits2byte() {
        assert_eq!(
            bits2byte(&vec![false, false, false, false, true, true, true, true]),
            15
        )
    }

    pub fn int2bytes(value: i32) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(((value >> 24) & 0xFF) as u8);
        bytes.push(((value >> 16) & 0xFF) as u8);
        bytes.push(((value >> 8) & 0xFF) as u8);
        bytes.push((value & 0xFF) as u8);
        bytes
    }

    #[test]
    fn test_int2bytes() {
        assert_eq!(int2bytes(1000), vec![0, 0, 3, 232])
    }

    pub fn bytes2int(bytes: &Vec<u8>) -> i32 {
        let mut value: i32 = 0;
        for i in 0..4 {
            value += (((*bytes)[i] & 0xFF) as i32) << (3 - i) * 8;
        }
        value
    }

    #[test]
    fn test_bytes2int() {
        assert_eq!(bytes2int(&vec![0, 0, 3, 233]), 1001)
    }

    pub fn short2bytes(value: i16) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(((value >> 8) & 0xFF) as u8);
        bytes.push((value & 0xFF) as u8);
        bytes
    }

    #[test]
    fn test_short2bytes() {
        assert_eq!(short2bytes(300), vec![1, 44])
    }

    pub fn bytes2short(bytes: &Vec<u8>) -> i16 {
        let mut value: i16 = 0;
        for i in 0..2 {
            value <<= 8;
            value |= ((*bytes)[i] & 0xFF) as i16;
        }
        value
    }

    #[test]
    fn test_bytes2short() {
        assert_eq!(bytes2short(&vec![1, 45]), 301)
    }

    pub fn connect(container: &mut Vec<u8>, appendence: &Vec<u8>) {
        for byte in appendence.iter() {
            container.push(*byte);
        }
    }
}
