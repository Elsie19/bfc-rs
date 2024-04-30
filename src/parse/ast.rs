use crate::parse::opcodes::OpCodes;
use crate::parse::opcodes::Tokens;
use anyhow::anyhow;

pub fn balance_brackets(program: &std::str::Chars) -> std::result::Result<(), anyhow::Error> {
    let left = program.clone().filter(|&n| n == ']').count();
    let right = program.clone().filter(|&n| n == '[').count();
    if left != right {
        Err(anyhow!(format!(
            "Could not balance brackets:\n'[': {}\n']': {}",
            right, left
        )))
    } else {
        Ok(())
    }
}

// BUG: `line_num` and `column_num` get reset when passed into `[`
pub fn generate_ast(program: &mut std::str::Chars) -> Vec<Tokens> {
    let mut out = vec![];
    let mut line_num = 1;
    let mut column_num = 0;
    while let Some(part) = program.next() {
        column_num += 1;
        match part {
            '>' => out.push(Tokens::new(OpCodes::Inc(1), (line_num, column_num))),
            '<' => out.push(Tokens::new(OpCodes::Dec(1), (line_num, column_num))),
            '+' => out.push(Tokens::new(OpCodes::Add(1), (line_num, column_num))),
            '-' => out.push(Tokens::new(OpCodes::Sub(1), (line_num, column_num))),
            '.' => out.push(Tokens::new(OpCodes::Output, (line_num, column_num))),
            // ',' => out.push(OpCodes::Input),
            ',' => out.push(Tokens::new(OpCodes::Input, (line_num, column_num))),
            '[' => {
                // out.push(OpCodes::Loop(generate_ast(program)));
                out.push(Tokens::new(
                    OpCodes::Loop(generate_ast(program)),
                    (line_num, column_num),
                ));
            }
            ']' => {
                break;
            }
            '\n' => {
                line_num += 1;
                column_num = 0;
            }
            _ => (), /* Comments probably */
        }
    }
    out
}
