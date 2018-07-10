use std::cell::RefCell;
use std::rc::Rc;

use canframe::CANFrame;
use encoder::Encoder;

pub struct Bus {
    content: Option<CANFrame>
}

impl Bus {
    pub fn new() -> Rc<RefCell<Bus>> {
        Rc::new(RefCell::new(
            Bus {
                content: None
            }
        ))
    }

    fn send(&mut self, input: CANFrame) -> () {
        //TODO: check header and choose data.
        self.content = Some(input)
    }

    fn recieve(&self) -> Option<&CANFrame> {
        self.content.as_ref()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_new() {
        let bus = Bus::new();
        assert!(bus.borrow().recieve().is_none())
    }

    #[test]
    fn test_bus_send_and_recieve() {
        let mut can_frame = CANFrame::new(5);
        can_frame.set_RTR_and_ctr_bits(6);
        can_frame.set_data(Encoder::encode("foobar").unwrap());
        can_frame.prepare_send();

        let mut ref_canframe = CANFrame::new(5);
        ref_canframe.set_RTR_and_ctr_bits(6);
        ref_canframe.set_data(Encoder::encode("foobar").unwrap());
        ref_canframe.prepare_send();

        let mut bus = Bus::new();
        bus.borrow_mut().send(can_frame);
        assert_eq!(ref_canframe, *bus.borrow().recieve().unwrap());
    }

    #[test]
    fn test_bus_send_twice() {
        assert!(false)
    }
}

