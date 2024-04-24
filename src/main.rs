mod args;
mod execute;
mod parse;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use args::args::Args;
use args::args::Commands;
use clap::Parser;
use execute::compile::compile;
use execute::interpret::interpret;
use execute::machine::Machine;
use parse::ast::{balance_brackets, generate_ast};
use parse::optimizer::{optimize, OptimizerStrategies};
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() {
    let args = Args::parse();
    let optimizings = vec![
        OptimizerStrategies::Contractions,
        // OptimizerStrategies::ClearLoop,
        OptimizerStrategies::DeadCode,
        OptimizerStrategies::PureCode,
    ];

    match &args.cmd {
        Commands::Shell {} => {
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
                        let ast =
                            optimize(generate_ast(&mut buffer.chars()), optimizings.to_owned());
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
        Commands::Compile { rest } => {
            let file_contents = fs::read_to_string(rest).expect("Could not read file");
            let mut file_contents = file_contents.chars();
            if let Err(nar) = balance_brackets(&file_contents) {
                eprintln!("{}", nar);
                std::process::exit(1);
            }
            let ast = generate_ast(&mut file_contents);
            let ast = optimize(ast, optimizings);
            let file_name = rest;
            let machine = Machine::new(30_000);
            println!(">> Compiling to IR...");
            let text = compile(&ast, &machine);
            let tmp_path = format!(
                "/tmp/bfc-rs-{}",
                file_name.file_name().unwrap().to_str().unwrap()
            );
            let mut tmp = File::create(&tmp_path).unwrap();
            write!(tmp, "{}", text).unwrap();
            let s_path = format!(
                "/tmp/bfc-rs-{}.s",
                file_name.file_name().unwrap().to_str().unwrap()
            );
            println!(">> Generating assembly...");
            Command::new("qbe")
                .args(["-o", &s_path, &tmp_path])
                .output()
                .expect("Could not run qbe");
            println!(">> Compiling assembly to final binary...");
            Command::new("cc")
                .args([
                    "-static",
                    s_path.as_str(),
                    "-o",
                    Path::new(&file_name)
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .unwrap_or("Unknown"),
                ])
                .output()
                .expect("Could not run cc");
            println!(">> Stripping binary...");
            Command::new("strip")
                .args([Path::new(&file_name)
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .unwrap_or("Unknown")])
                .output()
                .expect("Could not run strip");
            fs::remove_file(tmp_path).unwrap();
            fs::remove_file(s_path).unwrap();
        }
        Commands::Interpret { rest } => {
            let file_contents = fs::read_to_string(rest).expect("Could not read file");
            let mut file_contents = file_contents.chars();
            if let Err(nar) = balance_brackets(&file_contents) {
                eprintln!("{}", nar);
                std::process::exit(1);
            }
            let ast = generate_ast(&mut file_contents);
            let ast = optimize(ast, optimizings);
            let mut machine = Machine::new(30_000);
            interpret(&ast, &mut machine);
        }
    }
}
