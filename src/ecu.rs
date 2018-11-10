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

    pub fn send<'a>(&'a self, input: &str) -> Result<String, String> {
        let frame_sended = self.send_sub(input);
        self.check(&frame_sended)
    }

    fn send_sub(&self, input: &str) -> CANFrame {
        let mut canframe = CANFrame::new(self.id);
        let input_byte = Encoder::encode(input).unwrap();
        canframe.set_RTR_and_ctr_bits(input_byte.len());
        canframe.set_data(input_byte);
        canframe.prepare_send();
        let ret_val = canframe.clone();

        self.connection.borrow_mut().send(canframe);
        ret_val
    }

    fn check(& self, input_frame: &CANFrame) -> Result<String, String> {
        let bus = self.connection.borrow();
        let bus_frame = bus.recieve().ok_or("Bus is empty".to_string())?;
        if bus_frame == input_frame {
            Encoder::reverse(&input_frame.get_data()) 
                .map(|v| v.to_owned())
                .map_err(|e| e.to_string())
        } else {
            Err("Failed to send".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecu_send_and_check() {
        let bus = Bus::new();
        let ecu = ECU::new(1, &bus);
        let res = ecu.send("hoge");
        assert_eq!(res.unwrap().as_str(), "hoge")
    }

    #[test]
    fn test_ecu_sending_twice_1st_prior_to_2nd(){
        let bus = Bus::new();
        let ecu1 = ECU::new(1, &bus);
        let ecu2 = ECU::new(2, &bus);
        let res1 = ecu1.send("hoge");
        let res2 = ecu2.send("fuga");
        assert_eq!(res2.unwrap_err(), "Failed to send")
    }

    #[test]
    fn test_ecu_sending_twice_2nd_prior_to_1st(){
        let bus = Bus::new();
        let ecu1 = ECU::new(1, &bus);
        let ecu2 = ECU::new(2, &bus);
        let res2 = ecu2.send("fuga");
        let res1 = ecu1.send("hoge");
        assert_eq!(res1.unwrap().as_str(), "hoge")
    }
}
