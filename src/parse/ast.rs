use crate::parse::opcodes::OpCodes;

pub fn generate_ast(program: &mut std::str::Chars) -> Vec<OpCodes> {
    let mut out = vec![];
    while let Some(part) = program.next() {
        match part {
            '>' => out.push(OpCodes::Inc(1)),
            '<' => out.push(OpCodes::Dec(1)),
            '+' => out.push(OpCodes::Add(1)),
            '-' => out.push(OpCodes::Sub(1)),
            '.' => out.push(OpCodes::Output),
            ',' => out.push(OpCodes::Input),
            '[' => out.push(OpCodes::Loop(generate_ast(program))),
            ']' => break,
            _ => (), /* Comments probably */
        }
    }
    out
}
