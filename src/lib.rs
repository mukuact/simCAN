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
        // add id as a header
        self.connection.borrow_mut().send(input_byte)
    }

    pub fn receive(&self) -> (){
        let connection = self.connection.borrow();
        let recieved_content = connection.receive();
        println!("ECU{}: the bus content is", self.id);
        for i in 0..10 {
            println!("{:b}", &recieved_content[i]);
        }
    }
}

pub struct Encoder {
}

impl Encoder {
    pub fn encode(input: &str) -> Result<&[u8], String> {
        if !input.is_ascii() {
            return Err("This input cant encode. Only ascii charactor is accepted.".to_string())

        }

        let byte_input = input.as_bytes();
        println!("{:?}", byte_input);
        Ok(byte_input)
    }
}
