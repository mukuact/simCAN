extern crate bytes;
use std::cell::RefCell;
use std::rc::Rc;

use bus::Bus;
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

    pub fn send<'a>(&self, input: &'a str) -> Result<&'a str, &'static str> {
        self.send_sub(input);
        self.check(input)
    }

    fn send_sub(&self, input: &str) {
        let mut canframe = CANFrame::new(self.id);
        let input_byte = Encoder::encode(input).unwrap();
        canframe.set_RTR_and_ctr_bits(input_byte.len());
        canframe.set_data(input_byte);
        canframe.prepare_send();

        self.connection.borrow_mut().send(canframe)
    }

    fn check<'a> (& self, input_content: &'a str) -> Result<&'a str, &'static str> {
        let bus = self.connection.borrow();
        let bus_content = bus.recieve().ok_or("Bus is empty")?;
        println!("bus {:?}", bus_content.get_data());
        println!("in {:?}", Encoder::encode(input_content).unwrap());
        if bus_content.get_data() == Encoder::encode(input_content).unwrap() {
            Ok(input_content)
        } else {
            Err("Failed to send")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecu_send_and_check() {
        let bus = Bus::new();
        let mut ecu = ECU::new(1, &bus);
        let res = ecu.send("hoge");
        res.unwrap();
    }
}
