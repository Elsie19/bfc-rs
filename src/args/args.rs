use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// bfc is a brainfuck compiler/interpreter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compile program
    Compile {
        /// Input file
        #[clap(required = true)]
        rest: PathBuf,
    },

    /// Interpret program
    Interpret {
        /// Input file
        #[clap(required = true)]
        rest: PathBuf,
    },

    /// Launch shell
    Shell {},
}
