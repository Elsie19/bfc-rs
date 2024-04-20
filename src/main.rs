mod execute;
mod parse;

use std::env;
use std::fs;

use execute::interpret::interpret;
use execute::machine::Machine;
use parse::ast::generate_ast;
use parse::optimizer::{optimize, OptimizerStrategies};

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_contents = fs::read_to_string(&args[1]).expect("Could not read file");
    let mut file_contents = file_contents.chars();
    let optimizings = vec![
        OptimizerStrategies::Contractions,
        OptimizerStrategies::ClearLoop,
    ];
    println!(">> Generating AST...");
    let ast = generate_ast(&mut file_contents);
    println!(">> Optimizing AST...");
    let ast = optimize(ast, optimizings);
    println!(">> Running...");

    let mut machine = Machine::new(30_000);
    interpret(&ast, &mut machine);
}
