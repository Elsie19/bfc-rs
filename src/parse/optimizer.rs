use crate::parse::opcodes::OpCodes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerStrategies {
    Contractions,
    ClearLoop,
    Copy,
}

pub fn optimize(ast: Vec<OpCodes>, optimizers: Vec<OptimizerStrategies>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = ast.clone();
    if optimizers.contains(&OptimizerStrategies::Contractions) {
        new_ast = contract(new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::ClearLoop) {
        new_ast = clear(new_ast);
    }
    new_ast
}

fn contract(ast: Vec<OpCodes>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = vec![];
    let mut p = ast.iter().peekable();
    while let Some(op) = p.next() {
        match op {
            OpCodes::Add(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Add(counter as u8));
            }
            OpCodes::Sub(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Sub(counter as u8));
            }
            OpCodes::Inc(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Inc(counter as u8));
            }
            OpCodes::Dec(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Dec(counter as u8));
            }
            OpCodes::Loop(x) => {
                new_ast.push(OpCodes::Loop(contract(x.to_vec())));
            }
            _ => new_ast.push(op.clone()),
        }
    }
    new_ast
}

fn clear(ast: Vec<OpCodes>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = vec![];
    for part in ast {
        match part {
            OpCodes::Loop(ref x) => {
                if x.len() == 1 {
                    match x.get(0).unwrap() {
                        OpCodes::Add(_) | OpCodes::Sub(_) => new_ast.push(OpCodes::Clear),
                        _ => new_ast.push(part),
                    }
                } else {
                    new_ast.push(OpCodes::Loop(clear(x.to_vec())));
                }
            }
            _ => new_ast.push(part.clone()),
        }
    }
    new_ast
}
