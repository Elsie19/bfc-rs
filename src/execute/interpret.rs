use crate::execute::machine::Machine;
use crate::parse::opcodes::OpCodes;

pub fn interpret(ast: &Vec<OpCodes>, machine: &mut Machine) {
    for op in ast {
        match op {
            OpCodes::Inc(x) => {
                machine.increment(x.to_owned().into());
            }
            OpCodes::Dec(x) => {
                machine.decrement(x.to_owned().into());
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
