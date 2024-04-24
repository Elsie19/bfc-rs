use crate::parse::opcodes::OpCodes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerStrategies {
    Contractions,
    ClearLoop,
    DeadCode,
    PureCode,
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
    if optimizers.contains(&OptimizerStrategies::PureCode) {
        new_ast = remove_pure(new_ast);
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
                new_ast.push(OpCodes::Add(counter as u32));
            }
            OpCodes::Sub(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Sub(counter as u32));
            }
            OpCodes::Inc(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Inc(counter as u32));
            }
            OpCodes::Dec(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter = counter.saturating_add(1);
                    p.next();
                }
                new_ast.push(OpCodes::Dec(counter as u32));
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
            enummy @ OpCodes::Add(_)
            | enummy @ OpCodes::Sub(_)
            | enummy @ OpCodes::Inc(_)
            | enummy @ OpCodes::Dec(_) => {
                if p.peek().copied() == enummy.opposite().as_ref() {
                    p.next();
                } else {
                    new_ast.push(enummy.to_owned());
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
                    match x.first().unwrap() {
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

fn remove_pure(ast: Vec<OpCodes>) -> Vec<OpCodes> {
    let mut new_ast: Vec<OpCodes> = ast.clone();
    let mut pure_ast: Vec<OpCodes> = vec![];
    while let Some(op) = new_ast.pop() {
        match op {
            // Basically if we have codes at the end that cause side effects, we push that, but if
            // we don't, we push that to pure_ast instead. Later I might add a warning message
            // about the no-effect code.
            OpCodes::Input | OpCodes::Output | OpCodes::Loop { .. } => {
                new_ast.push(op);
                break;
            }
            _ => pure_ast.push(op),
        }
    }
    new_ast
}
