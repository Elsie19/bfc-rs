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

pub fn to_string_ast(program: Vec<OpCodes>) {
    for part in program {
        match part {
            OpCodes::Add(x) => print!("{:+<1$}", "", x.to_owned().into()),
            OpCodes::Sub(x) => print!("{:-<1$}", "", x.to_owned().into()),
            OpCodes::Inc(x) => print!("{:><1$}", "", x.to_owned().into()),
            OpCodes::Dec(x) => print!("{:<<1$}", "", x.to_owned().into()),
            OpCodes::Output => print!("."),
            OpCodes::Input => print!(","),
            OpCodes::Loop(x) => {
                print!("[");
                to_string_ast(x);
                print!("]");
            }
            OpCodes::Clear => continue,
        }
    }
}
