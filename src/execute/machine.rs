use std::io::{self, stdin, Read, Write};
use wrapnum::{wrap, WrapNum};

pub struct Machine {
    ptr: WrapNum<usize>,
    tape: Vec<u32>,
}

impl Machine {
    pub fn new(size: usize) -> Self {
        Machine {
            ptr: wrap!(size),
            tape: vec![0; size],
        }
    }

    pub fn get_size(&self) -> usize {
        self.tape.len()
    }

    pub fn set_byte(&mut self, num: u32) {
        self.tape[self.ptr] = num;
    }

    pub fn add(&mut self, num: u32) {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(num);
    }

    pub fn sub(&mut self, num: u32) {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(num);
    }

    pub fn increment(&mut self, num: usize) {
        self.ptr += num;
    }

    pub fn decrement(&mut self, num: usize) {
        self.ptr -= num;
    }

    pub fn input(&mut self) {
        let mut input: [u8; 1] = [0; 1];
        stdin()
            .read_exact(&mut input)
            .expect("Could not read stdin");
        io::stdout().flush().unwrap();
        self.tape[self.ptr] = input[0].into();
    }

    pub fn output(&self) {
        print!("{}", char::from_u32(self.tape[self.ptr]).unwrap());
        io::stdout().flush().unwrap();
    }

    pub fn get_byte(&self) -> u32 {
        self.tape[self.ptr]
    }
}
