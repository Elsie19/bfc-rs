use std::io::{stdin, Read};

pub struct Machine {
    ptr: usize,
    tape: Vec<u32>,
}

impl Machine {
    pub fn new(size: usize) -> Self {
        Machine {
            ptr: 0,
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
        self.ptr = self.ptr.wrapping_add(num);
    }

    pub fn decrement(&mut self, num: usize) {
        self.ptr = self.ptr.wrapping_sub(num);
    }

    pub fn input(&mut self) {
        let mut input: [u8; 4] = [0; 4];
        stdin()
            .read_exact(&mut input)
            .expect("Could not read stdin");
        self.tape[self.ptr] = u32::from_le_bytes(input);
    }

    pub fn output(&self) {
        print!("{}", char::from_u32(self.tape[self.ptr]).unwrap());
    }

    pub fn get_byte(&self) -> u32 {
        self.tape[self.ptr]
    }
}
