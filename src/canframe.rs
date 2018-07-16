extern crate bit_field;
use self::bit_field::BitField;
use std::cmp::Ordering;
use std::fmt;
use std::result::Result;


#[derive(Clone)]
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
        // reset RTR, IDE, r, data_length
        self.frame[0] &= !0x0007E000;
        // set RTR
        self.frame[0] |= 1 << (32-13) ;
        // set IDE, r
        self.frame[0] |= (1 << (32-14)) | (1 << (32-15)) ;
        // set data length
        self.frame[0] |= (data_length as u32) << (32-19);
    }

    pub fn set_data(&mut self, data: &[u8]) {
        for (i, onebyte) in (0..self.data_size()).zip(data.iter()) {
            match i {
                0 => self.frame[0] |= (*onebyte as u32) << 5,
                1 => {
                    self.frame[0] |= onebyte.get_bits(3..8) as u32 ;
                    self.frame[1] |= (onebyte.get_bits(0..3) as u32) << 29;
                },
                2 => self.frame[1] |= (*onebyte as u32) << 21,
                3 => self.frame[1] |= (*onebyte as u32) << 13,
                4 => self.frame[1] |= (*onebyte as u32) << 5,
                5 => { 
                    self.frame[1] |= onebyte.get_bits(3..8) as u32;
                    self.frame[2] |= (onebyte.get_bits(0..3) as u32) << 29;
                },
                6 => self.frame[2] |= (*onebyte as u32) << 21,
                7 => self.frame[2] |= (*onebyte as u32) << 13,
                // 8 => self.frame[2] |= (*onebyte as u32) << 1,
                _ => println!("fuga"),
            }
        }
    }

    pub fn get_data(&self) -> [u8; 8]{
        let data_len = self.data_size();
        let mut array: [u8; 8] = [0; 8];
        for i in 0..data_len {
            match i {
                0 => {
                    array[i] = self.frame[0].get_bits(5..13) as u8;
                },
                1 => {
                    let mut tmp = (self.frame[0].get_bits(0..5) << 3) as u8;
                    tmp |= self.frame[1].get_bits(29..32) as u8;
                    array[i] = tmp
                },
                2 => {
                    array[i] = self.frame[1].get_bits(21..29) as u8;
                },
                3 => {
                    array[i] = self.frame[1].get_bits(13..20) as u8;
                },
                4 => {
                    array[i] = self.frame[1].get_bits(5..12) as u8;
                },
                5 => {
                    let mut tmp = (self.frame[1].get_bits(0..5) << 3) as u8;
                    tmp |= self.frame[2].get_bits(29..32) as u8;
                    array[i] = tmp
                },
                6 => {
                    array[i] = self.frame[2].get_bits(21..29) as u8;
                },
                7 => {
                    array[i] = self.frame[2].get_bits(13..20) as u8;
                },
                _ => (),
            }
        }
        array
    }

    pub fn prepare_send(&mut self) {
        let mut cursor = 0;
        let mask = 0b11111;
        let mut masked_data;
        while cursor < 64*3 {
            match cursor {
                0...27 => {
                    masked_data = (self.frame[0] & (mask << (32 - cursor - 5))) >> (32 - cursor - 5);
                },
                28...31 => {
                    let first_frame = self.frame[0] & (mask >> (5 - (32 - cursor)));
                    let second_frame = (self.frame[1] & (mask << 27)) >> (27 + (32 - cursor));
                    masked_data = first_frame << (5 -(32 - cursor)) | second_frame;
                },
                32...59 => {
                    masked_data = (self.frame[1] & (mask << (64 - cursor - 5))) >> (64 - cursor - 5);
                },
                60...63 => {
                    let first_frame = self.frame[1] & (mask >> (5 - (64 - cursor)));
                    let second_frame = (self.frame[2] & (mask << 27)) >> (27 + (64 - cursor));
                    masked_data = first_frame << (5 -(64 - cursor)) | second_frame;
                },
                64...90 => {
                    masked_data = (self.frame[2] & (mask << (96 - cursor - 5))) >> (96 - cursor - 5);
                },
                _=> return,
            }
            match CANFrame::check_bit_change(&masked_data) {
                Ok(change_point) => {
                    cursor += change_point;
                },
                Err(bit) => {
                    self.add_bit_at(cursor+5, !bit);
                    cursor += 5;
                }
            }
        }
    }

    pub fn view(&self) -> &[u32] {
        &self.frame[..]
    }

    fn add_bit_at(&mut self, at: usize, bit: bool) {
        let rcursor = 96 - at;
        if rcursor > 0 {
            let data = self.frame[2];
            let rocal_cur = 32usize.saturating_sub(rcursor);
            let mask = 0xffffffff >> rocal_cur;
            self.frame[2] = data & !mask | (data & mask) >> 1;
            self.frame[2].set_bit(31-rocal_cur, bit);
        }
        if rcursor > 32 {
            let data = self.frame[1];
            let rocal_cur = 64usize.saturating_sub(rcursor);

            let lastbit = data.get_bit(0);
            self.frame[2].set_bit(31, lastbit);

            let mask = 0xffffffff >> rocal_cur;
            self.frame[1] = data & !mask | (data & mask) >> 1;
            self.frame[1].set_bit(31-rocal_cur, bit);
        }
        if rcursor > 64 {
            let data = self.frame[0];
            let rocal_cur = 96usize.saturating_sub(rcursor);

            let lastbit = data.get_bit(0);
            self.frame[1].set_bit(31, lastbit);

            let mask = 0xffffffff >> rocal_cur  ;
            self.frame[0] = data & !mask | (data & mask) >> 1;
            self.frame[0].set_bit(31-rocal_cur, bit);
        }
    }

    fn check_bit_change(data: &u32) -> Result<usize, bool> {
        let mut prev_bit = data.get_bit(0);
        for i in 1..5 {
            let bit = data.get_bit(i);
            if prev_bit != bit {
                return Ok(5-i)
            }
        }
        return Err(prev_bit)
    }

    fn data_size(&self) -> usize {
        let mut data_length = 0;
        data_length |= (self.frame[0] & 0x0001E000) >> 13;
        data_length as usize
    }
}

impl fmt::Display for CANFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}_{:032b}_{:032b}", self.frame[0], self.frame[1], self.frame[2],)
    }
}

impl fmt::Debug for CANFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}_{:X}_{:X}", self.frame[0], self.frame[1], self.frame[2])
    }
}

impl fmt::UpperHex for CANFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}_{:X}_{:X}", self.frame[0], self.frame[1], self.frame[2])
    }
}

impl PartialEq for CANFrame {
    fn eq(&self, other:&CANFrame) -> bool {
        self.frame == other.frame
    }
}

impl PartialOrd for CANFrame {
    fn partial_cmp(&self, other: &CANFrame) -> Option<Ordering> {
        let own_id = (self.frame[0] & 0x4FF00000) >> 20; 
        let other_id = (other.frame[0] & 0x4FF00000) >> 20; 
        Some(own_id.cmp(&other_id))
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
        assert_eq!(can_frame.view()[1], 0xECECACCE);
        assert_eq!(can_frame.view()[2], 0xACEC2000);
    }

    #[test]
    fn test_set_data_6byte_hogefuga() {
        let mut can_frame = CANFrame::new(101);
        can_frame.set_RTR_and_ctr_bits(6);
        can_frame.set_data("hogefuga".as_bytes());
        assert_eq!(can_frame.view()[0], 0x865ECD0D);
        assert_eq!(can_frame.view()[1], 0xECECACCE);
        assert_eq!(can_frame.view()[2], 0xA0000000);
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
        assert_eq!(can_frame.view()[0], 0x820F8C17);
        assert_eq!(can_frame.view()[1], 0xD8DC1041);
        assert_eq!(can_frame.view()[2], 0x04104104);
    }

    #[test]
    fn test_ord_id_11_vs_30() {
        let mut can_frame1 = CANFrame::new(11);
        let mut can_frame2 = CANFrame::new(30);
        
        let result = can_frame1 < can_frame2;
        assert_eq!(result, true);
    }

    #[test]
    fn test_ord_same_id_different_data () {
        let mut can_frame1 = CANFrame::new(11);
        let mut can_frame2 = CANFrame::new(11);
        
        let result_true = can_frame1 <= can_frame2;
        let result_false = can_frame1 < can_frame2;

        assert_eq!(result_true, true);
        assert_eq!(result_false, false);
    }

    #[test]
    fn test_get_data() {
        let mut can_frame = CANFrame::new(1);
        can_frame.set_RTR_and_ctr_bits(8);
        can_frame.set_data("hogefuga".as_bytes());

        assert_eq!(can_frame.get_data(), "hogefuga".as_bytes());
    }
}
