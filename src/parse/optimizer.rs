use crate::parse::opcodes::OpCodes;

use super::opcodes::Tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerStrategies {
    Contractions,
    ClearLoop,
    DeadCode,
    PureCode,
}

pub fn optimize(ast: &[Tokens], optimizers: &[OptimizerStrategies]) -> Vec<Tokens> {
    let mut new_ast: Vec<Tokens> = ast.to_owned();
    if optimizers.contains(&OptimizerStrategies::ClearLoop) {
        new_ast = clear(&new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::DeadCode) {
        new_ast = clear_dead_code(&new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::Contractions) {
        new_ast = contract(&new_ast);
    }
    if optimizers.contains(&OptimizerStrategies::PureCode) {
        new_ast = remove_pure(&new_ast);
    }
    new_ast
}

fn contract(ast: &[Tokens]) -> Vec<Tokens> {
    let mut new_ast: Vec<Tokens> = vec![];
    let mut p = ast.iter().peekable();
    while let Some(op) = p.next() {
        match op.get_type() {
            OpCodes::Add(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter += 1;
                    p.next();
                }
                new_ast.push(Tokens::new(
                    OpCodes::Add(u32::try_from(counter).unwrap()),
                    op.get_location().to_owned(),
                ));
            }
            OpCodes::Sub(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter += 1;
                    p.next();
                }
                new_ast.push(Tokens::new(
                    OpCodes::Sub(u32::try_from(counter).unwrap()),
                    op.get_location().to_owned(),
                ));
            }
            OpCodes::Inc(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter += 1;
                    p.next();
                }
                new_ast.push(Tokens::new(
                    OpCodes::Inc(u32::try_from(counter).unwrap()),
                    op.get_location().to_owned(),
                ));
            }
            OpCodes::Dec(x) => {
                let mut counter = *x as usize;
                while Some(op) == p.peek().copied() {
                    counter += 1;
                    p.next();
                }
                new_ast.push(Tokens::new(
                    OpCodes::Dec(u32::try_from(counter).unwrap()),
                    op.get_location().to_owned(),
                ));
            }
            OpCodes::Loop(x) => {
                new_ast.push(Tokens::new(
                    OpCodes::Loop(contract(x)),
                    op.get_location().to_owned(),
                ));
            }
            _ => new_ast.push(op.to_owned()),
        }
    }
    new_ast
}

fn clear_dead_code(ast: &[Tokens]) -> Vec<Tokens> {
    let mut new_ast: Vec<Tokens> = vec![];
    let mut counter = 0;

    let mut p = ast.iter().peekable();
    while let Some(part) = p.next() {
        match part.get_type() {
            OpCodes::Loop(x) => {
                // Remove empty loops `[]`
                if !x.is_empty() {
                    new_ast.push(Tokens::new(
                        OpCodes::Loop(x.to_vec()),
                        part.get_location().to_owned(),
                    ))
                }
            }
            OpCodes::Clear => {
                // Do we have `[+]` or `[-]` at the beginning?
                if counter != 0 {
                    // If not, push that mf
                    new_ast.push(Tokens::new(OpCodes::Clear, part.get_location().to_owned()))
                }
            }
            enummy @ (OpCodes::Add(_) | OpCodes::Sub(_) | OpCodes::Inc(_) | OpCodes::Dec(_)) => {
                // Option<Tokens>       Option<OpCodes>
                if p.peek().copied().is_some()
                    && p.peek().copied().unwrap().get_type() == enummy.opposite().as_ref().unwrap()
                {
                    p.next();
                } else {
                    new_ast.push(Tokens::new(
                        enummy.to_owned(),
                        part.get_location().to_owned(),
                    ));
                }
            }
            default => new_ast.push(Tokens::new(
                default.to_owned(),
                part.get_location().to_owned(),
            )),
        }
        counter += 1;
    }
    new_ast
}

fn clear(ast: &[Tokens]) -> Vec<Tokens> {
    let mut new_ast: Vec<Tokens> = vec![];
    for part in ast {
        match part.get_type() {
            OpCodes::Loop(ref x) => {
                // Do we have `[x]`
                if x.len() == 1 {
                    match x.first().unwrap().get_type() {
                        // Only match on possible clear values
                        OpCodes::Add(_) | OpCodes::Sub(_) => new_ast
                            .push(Tokens::new(OpCodes::Clear, part.get_location().to_owned())),
                        _ => new_ast.push(part.clone()),
                    }
                } else {
                    new_ast.push(Tokens::new(
                        OpCodes::Loop(clear(x)),
                        part.get_location().to_owned(),
                    ));
                }
            }
            _ => new_ast.push(part.to_owned()),
        }
    }
    new_ast
}

fn remove_pure(ast: &[Tokens]) -> Vec<Tokens> {
    let mut new_ast: Vec<Tokens> = ast.to_owned();
    let mut pure_ast: Vec<Tokens> = vec![];
    while let Some(op) = new_ast.pop() {
        match op.get_type() {
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
