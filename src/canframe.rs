extern crate bit_field;
use self::bit_field::BitField;

pub struct CANFrame {
    frame: [i32; 3],
}

impl CANFrame{
    pub fn new(id: usize) -> CANFrame {
        let mut sender: i32 = id as i32;
        sender = sender << 20;
        let length = i32::bit_length();
        sender.set_bit(length-1, true);

        CANFrame {
            frame: [sender, 0, 0]
        }
    }

    pub fn set_RTR_and_ctr_bits(&mut self, data_length: usize) {
        // set RTR
        self.frame[0].set_bit(32-13, true);
        // set IDE, r
        self.frame[0].set_bits((32-14)..(32-15), 0b11);
        // set data length
        self.frame[0].set_bits((32-16)..(32-18), data_length as i32);
    }

    pub fn print(&self) {
        for f in self.frame.iter() {
            print!("{:b}", f);
        }
        println!("");
    }
}
