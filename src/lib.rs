mod canframe;
pub mod ecu;
mod encoder;

use std::cell::RefCell;
use std::io::Cursor;
use std::io::prelude::*;
use std::rc::Rc;

pub struct Bus {
    content: Cursor<Vec<u8>>,
}

impl Bus {
    pub fn new() -> Rc<RefCell<Bus>> {
        Rc::new(RefCell::new(
            Bus {
                content: Cursor::new(vec![0; 10])
            }
        ))
    }

    fn send(&mut self, input: &[u8]) -> () {
        //TODO: check header and choose data.
        self.content.set_position(0);
        self.content.write(input);
    }

    fn receive<'a>(&'a self) -> &'a [u8] {
        &self.content.get_ref()
    }
}



