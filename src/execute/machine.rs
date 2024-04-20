use std::io::{stdin, Read};

pub struct Machine {
    ptr: usize,
    tape: Vec<u8>,
}

impl Machine {
    pub fn new(size: usize) -> Self {
        Machine {
            ptr: 0,
            tape: vec![0; size],
        }
    }

    pub fn set_byte(&mut self, num: u8) {
        self.tape[self.ptr] = num;
    }

    pub fn add(&mut self, num: u8) {
        self.tape[self.ptr] = self.tape[self.ptr].saturating_add(num);
    }

    pub fn sub(&mut self, num: u8) {
        self.tape[self.ptr] = self.tape[self.ptr].saturating_sub(num);
    }

    pub fn increment(&mut self, num: usize) {
        self.ptr = self.ptr.saturating_add(num);
    }

    pub fn decrement(&mut self, num: usize) {
        self.ptr = self.ptr.saturating_sub(num);
    }

    pub fn input(&mut self) {
        let mut input: [u8; 1] = [0; 1];
        stdin()
            .read_exact(&mut input)
            .expect("Could not read stdin");
        self.tape[self.ptr] = input[0].into();
    }

    pub fn output(&self) {
        print!("{}", self.tape[self.ptr] as char);
    }

    pub fn get_byte(&self) -> u8 {
        self.tape[self.ptr]
    }
}
