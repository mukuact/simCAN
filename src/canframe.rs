extern crate bit_field;
use self::bit_field::BitField;
use std::fmt;

pub struct CANFrame {
    frame: [i32; 3],
}

impl CANFrame{
    pub fn new(id: usize) -> CANFrame {
        if id > 0x7FF || id < 0 {
            panic!();
        }

        let mut sender: i32 = id as i32;
        sender = sender << 20;
        let length = i32::bit_length();
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
        self.frame[0] |= (data_length as i32) << (32-19);
    }

    pub fn set_data(&mut self, data: &[u8]) {
        for (i, onebyte) in data.iter().enumerate() {
            match i {
                0 => self.frame[0] |= (*onebyte as i32) << 5,
                1 => {
                    println!("{:b}", onebyte.get_bits(3..8));
                    self.frame[0] |= (onebyte.get_bits(3..8) as i32) ;
                    println!("{:b}", onebyte.get_bits(0..3));
                    self.frame[1] |= (onebyte.get_bits(0..3) as i32) << 27;
                },
                2 => self.frame[1] |= (*onebyte as i32) << 19,
                3 => self.frame[1] |= (*onebyte as i32) << 11,
                4 => self.frame[1] |= (*onebyte as i32) << 3,
                5 => { 
                    self.frame[1] |= (onebyte.get_bits(5..8) as i32);
                    self.frame[2] |= (onebyte.get_bits(0..5) as i32) << 25;
                },
                6 => self.frame[2] |= (*onebyte as i32) << 17,
                7 => self.frame[2] |= (*onebyte as i32) << 9,
                8 => self.frame[2] |= (*onebyte as i32) << 1,
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
        write!(f, "{:b}_{:b}_{:b}", self.frame[0], self.frame[1], self.frame[2],)
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
    fn make_CANFrame_id_over() {
        let can_frame = CANFrame::new(2100);
    }

    #[test]
    fn prepare_id101_8bytes_data (){
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(8);
        assert_eq!(can_frame.view()[0], 0b10000110010111110000000000000000)
    }

    #[test]
    fn prepare_id101_6bytes_data (){
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(6);
        println!("{}", can_frame);
        assert_eq!(can_frame.view()[0], 0b10000110010111101100000000000000)
    }

    #[test]
    #[should_panic]
    fn prepare_10byte_data () {
        let mut can_frame = CANFrame::new(4);
        can_frame.set_RTR_and_ctr_bits(10);
    }

    #[test]
    fn test_set_data_8byte_hogefuga (){
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(8);
        can_frame.set_data("hogefuga".as_bytes());
        assert_eq!(can_frame.view()[0], 0x865F0D0D);
        assert_eq!(can_frame.view()[1], 0x3B3B2B33);
        assert_eq!(can_frame.view()[2], 0x2ACEC200);
    }
}
