use crate::parse::opcodes::OpCodes;

use super::machine::Machine;

pub fn compile(ast: &Vec<OpCodes>, machine: &Machine) -> String {
    let mut output = format!(
        "#include <stdio.h>\nint main(void) {{\nchar mem[{}] = {{0}};\nint p = 0;\n",
        machine.get_size()
    );
    output += &_compile(ast, &machine);
    output += "}";
    output
}

fn _compile(ast: &Vec<OpCodes>, machine: &Machine) -> String {
    let mut output = String::new();
    for op in ast {
        match op {
            OpCodes::Inc(x) => {
                output += &format!("p += {};\n", x).to_string();
            }
            OpCodes::Dec(x) => {
                output += &format!("p -= {};\n", x).to_string();
            }
            OpCodes::Clear => {
                output += "mem[p] = 0;\n";
            }
            OpCodes::Add(x) => {
                output += &format!("mem[p] += {};\n", x).to_string();
            }
            OpCodes::Sub(x) => {
                output += &format!("mem[p] -= {};\n", x).to_string();
            }
            OpCodes::Input => {
                output += "mem[p] = getchar();\n";
            }
            OpCodes::Output => {
                output += "putchar(mem[p]);\n";
            }
            OpCodes::Loop(x) => {
                output += "while (mem[p]) {\n";
                output += &_compile(x, machine);
                output += "}\n";
            }
        }
    }
    output
}
