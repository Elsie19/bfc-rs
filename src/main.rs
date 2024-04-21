mod execute;
mod parse;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use execute::interpret::interpret;
use execute::machine::Machine;
use parse::ast::generate_ast;
use parse::optimizer::{optimize, OptimizerStrategies};

#[derive(clap::ValueEnum, Clone, Debug)]
enum OptimizerStrategiesClap {
    /// Contract
    Contract,
    /// Clear
    Clear,
}

/// bfc is a brainfuck compiler/interpreter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// Run with interpreter
    #[arg(short, long, conflicts_with = "compile", default_value_t = true)]
    interpret: bool,

    /// Compile to assembly
    #[arg(short, long, conflicts_with = "interpret")]
    compile: bool,

    /// Input file
    #[clap(required = true)]
    rest: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let file_contents = fs::read_to_string(args.rest.get(0).unwrap()).expect("Could not read file");
    let mut file_contents = file_contents.chars();
    let optimizings = vec![
        OptimizerStrategies::Contractions,
        OptimizerStrategies::ClearLoop,
    ];
    let ast = generate_ast(&mut file_contents);
    let ast = optimize(ast, optimizings);

    if args.compile {
        todo!("Make compiling work");
    } else if args.interpret {
        let mut machine = Machine::new(30_000);
        interpret(&ast, &mut machine);
    }
}
