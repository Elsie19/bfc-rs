use crate::execute::machine::Machine;
use crate::parse::opcodes::OpCodes;
use crate::parse::opcodes::Tokens;

pub fn interpret(ast: &Vec<Tokens>, machine: &mut Machine) {
    for op in ast {
        match op.get_type() {
            OpCodes::Inc(x) => {
                machine.increment(*x as usize);
            }
            OpCodes::Dec(x) => {
                machine.decrement(*x as usize);
            }
            OpCodes::Clear => {
                machine.set_byte(0);
            }
            OpCodes::Add(x) => {
                machine.add(*x);
            }
            OpCodes::Sub(x) => {
                machine.sub(*x);
            }
            OpCodes::Input => {
                machine.input();
            }
            OpCodes::Output => {
                machine.output();
            }
            OpCodes::Loop(x) => {
                while machine.get_byte() != 0 {
                    interpret(x, machine);
                }
            }
        }
    }
}
