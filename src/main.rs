mod execute;
mod parse;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use execute::compile::compile;
use execute::interpret::interpret;
use execute::machine::Machine;
use parse::ast::{balance_brackets, generate_ast};
use parse::optimizer::{optimize, OptimizerStrategies};
use reedline::{DefaultPrompt, Reedline, Signal};

/// bfc is a brainfuck compiler/interpreter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// Run with interpreter
    #[arg(short, long, conflicts_with = "compile", default_value_t = true)]
    interpret: bool,

    /// Run interactive shell
    #[arg(short, long, conflicts_with_all = ["compile", "interpret"], default_value_t = false)]
    shell: bool,

    /// Compile to assembly
    #[arg(short, long, conflicts_with = "interpret")]
    compile: bool,

    /// Input file
    #[clap(required_unless_present = "shell")]
    rest: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let optimizings = vec![
        OptimizerStrategies::Contractions,
        OptimizerStrategies::ClearLoop,
        OptimizerStrategies::DeadCode,
        OptimizerStrategies::PureCode,
    ];

    if args.shell {
        let mut line_editor = Reedline::create();
        let prompt = DefaultPrompt::new(
            reedline::DefaultPromptSegment::Empty,
            reedline::DefaultPromptSegment::CurrentDateTime,
        );
        loop {
            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    if buffer.to_lowercase() == "help" {
                        println!("\n# Brainfuck basics:");
                        println!("    >  Increment data pointer by one");
                        println!("    <  Decrement data pointer by one");
                        println!("    +  Increment byte at data pointer by one");
                        println!("    -  Decrement byte at data pointer by one");
                        println!("    .  Output byte at data pointer");
                        println!("    ,  Accent one byte, store it at the data pointer");
                        println!("    [  If byte at data pointer is zero, move to ']'");
                        println!("    ]  If byte at data pointer is nonzero, move to '['");
                        println!("\n# Example program:");
                        println!("    ++++++++[>++++[>++>+++>+++>+<<<<-]\n    >+>+>->>+[<]<-]>>.>---.+++++++..++\n    +.>>.<-.<.+++.------.--------.>>+.>++.");
                    }
                    let ast = optimize(generate_ast(&mut buffer.chars()), optimizings.to_owned());
                    let mut machine = Machine::new(30_000);
                    interpret(&ast, &mut machine);
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("\nQuitting! Hope you aren't insane yet!");
                    std::process::exit(130);
                }
                x => {
                    println!("Event: {:?}", x);
                }
            }
        }
    }

    let file_contents = fs::read_to_string(args.rest.get(0).unwrap()).expect("Could not read file");
    let mut file_contents = file_contents.chars();
    match balance_brackets(&file_contents) {
        Err(nar) => {
            eprintln!("{}", nar);
            std::process::exit(1);
        }
        _ => (),
    }
    let ast = generate_ast(&mut file_contents);
    let ast = optimize(ast, optimizings);

    if args.compile {
        let machine = Machine::new(30_000);
        let text = compile(&ast, &machine);
        let path = args.rest.get(0).unwrap();
        let c_file = path.file_stem().unwrap().to_str().unwrap().to_string() + ".c";
        let compiled_file = path.file_stem().unwrap().to_str().unwrap();
        let mut output = File::create(&c_file).unwrap();
        write!(output, "{}", text).unwrap();
        Command::new("gcc")
            .args(["-O0", &c_file, "-o", compiled_file])
            .output()
            .expect("Could not run 'gcc'");
        fs::remove_file(&c_file).unwrap();
    } else if args.interpret {
        let mut machine = Machine::new(30_000);
        interpret(&ast, &mut machine);
    }
}
