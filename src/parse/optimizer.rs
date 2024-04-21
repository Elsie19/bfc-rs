use crate::parse::opcodes::OpCodes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerStrategies {
    Contractions,
    ClearLoop,
    DeadCode,
}

pub fn optimize(ast: Vec<OpCodes>, optimizers: Vec<OptimizerStrategies>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = ast.clone();
    if optimizers.contains(&OptimizerStrategies::ClearLoop) {
        new_ast = clear(new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::DeadCode) {
        new_ast = clear_dead_code(new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::Contractions) {
        new_ast = contract(new_ast);
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
            _ => new_ast.push(op.to_owned()),
        }
    }
    new_ast
}

fn clear_dead_code(ast: Vec<OpCodes>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = vec![];
    let mut counter = 0;

    let mut p = ast.iter().peekable();
    while let Some(part) = p.next() {
        match part {
            OpCodes::Loop(x) => {
                // Remove empty loops `[]`
                if !x.is_empty() {
                    new_ast.push(OpCodes::Loop(x.to_vec()));
                }
            }
            OpCodes::Clear => {
                // Do we have `[+]` or `[-]` at the beginning?
                if counter != 0 {
                    // If not, push that mf
                    new_ast.push(OpCodes::Clear);
                }
            }
            // Ok this will get messy because... reasons, but if we detect opposite tokens next to
            // each other, we can yoink them
            OpCodes::Add(x) => {
                if p.peek().copied() == Some(&OpCodes::Sub(*x)) {
                    p.next();
                } else {
                    new_ast.push(OpCodes::Add(*x));
                }
            }
            OpCodes::Sub(x) => {
                if p.peek().copied() == Some(&OpCodes::Add(*x)) {
                    p.next();
                } else {
                    new_ast.push(OpCodes::Sub(*x));
                }
            }
            OpCodes::Inc(x) => {
                if p.peek().copied() == Some(&OpCodes::Dec(*x)) {
                    p.next();
                } else {
                    new_ast.push(OpCodes::Inc(*x));
                }
            }
            OpCodes::Dec(x) => {
                if p.peek().copied() == Some(&OpCodes::Inc(*x)) {
                    p.next();
                } else {
                    new_ast.push(OpCodes::Dec(*x));
                }
            }
            default => new_ast.push(default.to_owned()),
        }
        counter += 1;
    }
    new_ast
}

fn clear(ast: Vec<OpCodes>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = vec![];
    for part in ast {
        match part {
            OpCodes::Loop(ref x) => {
                // Do we have `[x]`
                if x.len() == 1 {
                    match x.get(0).unwrap() {
                        // Only match on possible clear values
                        OpCodes::Add(_) | OpCodes::Sub(_) => new_ast.push(OpCodes::Clear),
                        _ => new_ast.push(part),
                    }
                } else {
                    new_ast.push(OpCodes::Loop(clear(x.to_vec())));
                }
            }
            _ => new_ast.push(part.to_owned()),
        }
    }
    new_ast
}
