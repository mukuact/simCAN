extern crate bytes;
use self::bytes::{Bytes, BytesMut, BufMut};
use std::cell::RefCell;
use std::rc::Rc;

use Bus;
use canframe::CANFrame;
use encoder::Encoder;

pub struct ECU {
    id: usize,
    connection: Rc<RefCell<Bus>>,
}

impl ECU {
    pub fn new(id: usize, bus: &Rc<RefCell<Bus>>) -> ECU {
        ECU {
            id,
            connection: Rc::clone(&bus)
        }
    }

    pub fn send(&mut self, input: &str) -> (){
        let input_byte = Encoder::encode(input).unwrap();
        let mut buf = BytesMut::with_capacity(input_byte.len()+1024);
        buf.put(self.id as u8);
        buf.put(input_byte);
        // add id as a header
        self.connection.borrow_mut().send(&buf.take());
    }

    pub fn send_dataframe(&self, input: &str) -> bool {
        let mut frame = CANFrame::new(self.id);
        let input_byte = Encoder::encode(input).unwrap();
        frame.set_RTR_and_ctr_bits(input_byte.len());
        frame.print();
        true
    }

    pub fn receive(&self) -> (){
        let connection = self.connection.borrow();
        let recieved_content = connection.receive();
        print!("ECU{}: the bus content is ", self.id);
        println!("{}", Encoder::reverse(recieved_content).unwrap());
        for a_byte in recieved_content {
            println!("{:07b}", a_byte);
        }

    }


    pub fn zero_pudding(id: usize, num: usize) -> Bytes {
        let mut buf = BytesMut::with_capacity(num);

        let digits: usize = ((id as f32).log2().floor() + 1.0) as usize ;
        let len  = num - digits;

        for i in 0..len {
            buf.put(0x00 as u8);
        }
        buf.put(id as u8);
        let result = buf.freeze();
        result
    }
}
