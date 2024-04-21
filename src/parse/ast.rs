use crate::parse::opcodes::OpCodes;
use anyhow::anyhow;

pub fn balance_brackets(program: &std::str::Chars) -> std::result::Result<(), anyhow::Error> {
    let left = program.clone().filter(|&n| n == ']').count();
    let right = program.clone().filter(|&n| n == '[').count();
    if left != right {
        return Err(anyhow!(format!(
            "Could not balance brackets:\n'[': {}\n']': {}",
            right, left
        )));
    } else {
        Ok(())
    }
}

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
            '[' => {
                out.push(OpCodes::Loop(generate_ast(program)));
            }
            ']' => {
                break;
            }
            _ => (), /* Comments probably */
        }
    }
    out
}
