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
    #[clap(alias = "c")]
    Compile {
        /// Emit IR instead of compiling
        #[arg(short, long, default_value_t = false)]
        emit_ir: bool,

        /// Force dynamic linking
        #[arg(short, long, default_value_t = false)]
        dynamic: bool,

        /// Input file
        #[clap(required = true)]
        rest: PathBuf,
    },

    /// Interpret program
    #[clap(alias = "i")]
    Interpret {
        /// Input file
        #[clap(required = true)]
        rest: PathBuf,
    },

    /// Launch shell
    #[clap(alias = "s")]
    Shell {},
}
