#![feature(str_split_as_str)]
mod color;
mod instruction;
mod l_system;
mod program_state;

use crate::instruction::Instruction;
use crate::program_state::ProgramState;
use std::boxed::Box;
use std::env;
use std::fs::{self, File};
use std::io::Result as IoResult;
use std::io::Write;
use clap::{Args, Parser, Subcommand};

fn save_program(code: &Vec<Instruction>, filename: &str) -> IoResult<()> {
    let mut buffer = File::create(filename)?;
    for line in code {
        write!(buffer, "{}\n", line)?;
    }
    Ok(())
}

#[derive(Parser)]
#[clap(author = "May Lawver", version, about = "A pseudo-assembly turtle graphics language.", long_about = None)]
struct Command {
    #[clap(subcommand)]
    which: PenplotCommand
}

impl Command {
    fn run(&self) {
        match &self.which {
            PenplotCommand::Run(args) => args.run()
        }
    }
}

#[derive(Subcommand)]
enum PenplotCommand {
    Run(RunArgs)
}

/// Run a specified program and render its output to file.
#[derive(Args)]
struct RunArgs {
    /// Filename of source code to run
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    /// Filename to save the resulting image as
    output: String,
    /// Width of canvas
    #[clap(long, default_value_t = 512)]
    width: usize,
    /// Height of canvas
    #[clap(long, default_value_t = 512)]
    height: usize
}

impl RunArgs {
    fn run(&self) {
        let mut program = ProgramState::new(self.width, self.height);
        let commands = Instruction::parse_program(
            fs::read_to_string(&self.input).expect("Something went wrong reading the file"),
        );
        program.execute(commands.expect("Error parsing code"));
        program.save_buffer(&self.output);
    }
}

fn main() {
    let command = Command::parse();
    command.run();
}
