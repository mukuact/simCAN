use std::str;
use std::str::Utf8Error;

pub struct Encoder {
}

impl Encoder {
    pub fn encode(input: &str) -> Result<&[u8], String> {
        if !input.is_ascii() {
            return Err("This input cant encode. Only ascii charactor is accepted.".to_string())

        }

        let byte_input = input.as_bytes();
        // println!("{:?}", byte_input);
        Ok(byte_input)
    }

    pub fn reverse(input: &[u8]) -> Result<&str, Utf8Error> {
        let mut end = 0;
        for (i, char_) in input.iter().rev().enumerate() {
            if char_ != &(0 as u8) {
                end = input.len() - i;
                break;
            }
        }
        str::from_utf8(&input[..end])
    }
}
