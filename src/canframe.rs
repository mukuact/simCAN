extern crate bit_field;
use self::bit_field::BitField;
use std::fmt;

pub struct CANFrame {
    frame: [u32; 3],
}

impl CANFrame{
    pub fn new(id: usize) -> CANFrame {
        if id > 0x7FF {
            panic!();
        }

        let mut sender: u32 = id as u32;
        sender = sender << 20;
        let length = u32::bit_length();
        sender.set_bit(length-1, true);

        CANFrame {
            frame: [sender, 0, 0]
        }
    }

    pub fn set_RTR_and_ctr_bits(&mut self, data_length: usize) {
        if data_length > 8 {
            panic!("data_length must be within 8bytes");
        }
        // set RTR
        self.frame[0] |= 1 << (32-13) ;
        // set IDE, r
        self.frame[0] |= (1 << (32-14)) | (1 << (32-15)) ;
        // set data length
        self.frame[0] |= (data_length as u32) << (32-19);
    }

    pub fn set_data(&mut self, data: &[u8]) {
        for (i, onebyte) in data.iter().enumerate() {
            match i {
                0 => self.frame[0] |= (*onebyte as u32) << 5,
                1 => {
                    self.frame[0] |= (onebyte.get_bits(3..8) as u32) ;
                    self.frame[1] |= (onebyte.get_bits(0..3) as u32) << 27;
                },
                2 => self.frame[1] |= (*onebyte as u32) << 19,
                3 => self.frame[1] |= (*onebyte as u32) << 11,
                4 => self.frame[1] |= (*onebyte as u32) << 3,
                5 => { 
                    self.frame[1] |= (onebyte.get_bits(5..8) as u32);
                    self.frame[2] |= (onebyte.get_bits(0..5) as u32) << 25;
                },
                6 => self.frame[2] |= (*onebyte as u32) << 17,
                7 => self.frame[2] |= (*onebyte as u32) << 9,
                // 8 => self.frame[2] |= (*onebyte as u32) << 1,
                _ => println!("fuga"),
            }
            println!("{}: {:b}", i, onebyte);
        }
    }

    pub fn view(&self) -> &[i32] {
        &self.frame[..]
    }
}

impl fmt::Display for CANFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}_{:032b}_{:032b}", self.frame[0], self.frame[1], self.frame[2],)
    }
}
impl fmt::UpperHex for CANFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}_{:X}_{:X}", self.frame[0], self.frame[1], self.frame[2])
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_CANFrame_id_1() {
        let can_frame = CANFrame::new(1);
        assert_eq!(can_frame.view()[0], 0b10000000000100000000000000000000)
    }

    #[test]
    fn make_CANFrame_id_100() {
        let can_frame = CANFrame::new(101);
        assert_eq!(can_frame.view()[0], 0b10000110010100000000000000000000)
    }

    #[test]
    #[should_panic]
    fn make_CANFrame_id_over11bit() {
        let can_frame = CANFrame::new(2100);
    }

    #[test]
    fn prepare_id101_8bytes_data() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(8);
        assert_eq!(can_frame.view()[0], 0b10000110010111110000000000000000)
    }

    #[test]
    fn prepare_id101_6bytes_data() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(6);
        assert_eq!(can_frame.view()[0], 0b10000110010111101100000000000000)
    }

    #[test]
    fn set_RTRs_twice() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(7);
        assert_eq!(can_frame.view()[0], 0b10000110010111101110000000000000);
        can_frame.set_RTR_and_ctr_bits(5);
        assert_eq!(can_frame.view()[0], 0b10000110010111101010000000000000);
    }

    #[test]
    #[should_panic]
    fn prepare_10byte_data() {
        let mut can_frame = CANFrame::new(4);
        can_frame.set_RTR_and_ctr_bits(10);
    }

    #[test]
    fn test_set_data_8byte_hogefuga() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(8);
        can_frame.set_data("hogefuga".as_bytes());
        assert_eq!(can_frame.view()[0], 0x865F0D0D);
        assert_eq!(can_frame.view()[1], 0x3B3B2B33);
        assert_eq!(can_frame.view()[2], 0x2ACEC200);
    }

    #[test]
    fn test_set_data_6byte_hogefuga() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(6);
        can_frame.set_data("hogefuga".as_bytes());
        assert_eq!(can_frame.view()[0], 0x865ECD0D);
        assert_eq!(can_frame.view()[1], 0x3B3B2B33);
        assert_eq!(can_frame.view()[2], 0x2A000000);
    }

    #[test]
    fn test_size() {
        let mut can_frame = CANFrame::new(1);
        can_frame.set_RTR_and_ctr_bits(8);
        assert_eq!(can_frame.data_size(), 8);
        can_frame.set_RTR_and_ctr_bits(6);
        assert_eq!(can_frame.data_size(), 6);
    }

    #[test]
    fn test_bitstuffing() {
        let data: [u8; 2] = [0x3, 0xE0];
        let mut can_frame = CANFrame::new(1);
        can_frame.set_RTR_and_ctr_bits(2);
        can_frame.set_data(&data);
        can_frame.prepare_send();
        assert_eq!(can_frame.view()[0], 0x820F8827);
        assert_eq!(can_frame.view()[1], 0xC1041041);
    }

    #[test]
    fn test_bitstuffing2() {
        let data: [u8; 3] = [0x3, 0xF8, 0xDC];
        let mut can_frame = CANFrame::new(1);
        can_frame.set_RTR_and_ctr_bits(3);
        can_frame.set_data(&data);
        can_frame.prepare_send();
        println!("{:X}", can_frame);
        assert_eq!(can_frame.view()[0], 0x820F8C17);
        assert_eq!(can_frame.view()[1], 0xD83B8208);
        assert_eq!(can_frame.view()[2], 0x20820820);
    }
}
