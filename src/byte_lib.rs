pub mod processor {
    pub fn byte2bits(byte: u8) -> [bool; 8] {
        let mut bits = [false; 8];
        for pos in 0..8 {
            bits[pos] = ((byte >> (7 - pos)) & 0x1) != 0;
        }
        bits
    }

    #[test]
    fn test_byte2bits() {
        assert_eq!(
            byte2bits(7),
            [false, false, false, false, false, true, true, true]
        );
    }

    pub fn bits2byte(bits: &[bool]) -> u8 {
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
            bits2byte(&[false, false, false, false, true, true, true, true]),
            15
        )
    }

    pub fn int2bytes(value: i32) -> [u8; 4] {
        let mut bytes = [0; 4];
        bytes[0] = ((value >> 24) & 0xFF) as u8;
        bytes[1] = ((value >> 16) & 0xFF) as u8;
        bytes[2] = ((value >> 8) & 0xFF) as u8;
        bytes[3] = (value & 0xFF) as u8;
        bytes
    }

    #[test]
    fn test_int2bytes() {
        assert_eq!(int2bytes(1000), [0, 0, 3, 232])
    }

    pub fn bytes2int(bytes: &[u8]) -> i32 {
        let mut value: i32 = 0;
        for i in 0..4 {
            value += (((*bytes)[i] & 0xFF) as i32) << (3 - i) * 8;
        }
        value
    }

    #[test]
    fn test_bytes2int() {
        assert_eq!(bytes2int(&[0, 0, 3, 233]), 1001)
    }

    pub fn short2bytes(value: i16) -> [u8; 2] {
        let mut bytes = [0; 2];
        bytes[0] = ((value >> 8) & 0xFF) as u8;
        bytes[1] = (value & 0xFF) as u8;
        bytes
    }

    #[test]
    fn test_short2bytes() {
        assert_eq!(short2bytes(300), [1, 44])
    }

    pub fn bytes2short(bytes: &[u8]) -> i16 {
        let mut value: i16 = 0;
        for i in 0..2 {
            value <<= 8;
            value |= ((*bytes)[i] & 0xFF) as i16;
        }
        value
    }

    #[test]
    fn test_bytes2short() {
        assert_eq!(bytes2short(&[1, 45]), 301)
    }

    pub fn connect(container: &mut Vec<u8>, appendence: &Vec<u8>) {
        for byte in appendence.iter() {
            container.push(*byte);
        }
    }
}
