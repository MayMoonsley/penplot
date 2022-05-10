mod color;
mod instruction;
mod l_system;
mod parsing;
mod program_state;

use crate::instruction::Instruction;
use crate::program_state::ProgramState;
use std::fs::{self, File};
use std::io::Result as IoResult;
use std::io::Write;
use clap::{Args, Parser, Subcommand};

fn save_program(code: &[Instruction], filename: &str) -> IoResult<()> {
    let mut buffer = File::create(filename)?;
    for line in code {
        writeln!(buffer, "{}", line)?;
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
            PenplotCommand::Run(args) => args.run(),
            PenplotCommand::Fractal(args) => args.run()
        }
    }
}

#[derive(Subcommand)]
enum PenplotCommand {
    Run(RunArgs),
    Fractal(FractalArgs)
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
        let commands = parsing::parse_program(
            fs::read_to_string(&self.input).expect("Something went wrong reading the file"),
        );
        program.execute(commands.expect("Error parsing code"));
        program.save_buffer(&self.output);
    }
}

/// Iterate a specified L system and save its code output to a file
#[derive(Args)]
struct FractalArgs {
    /// Filename of L system specification
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    /// Filename to save the resulting code as
    output: String,
    #[clap(short, long)]
    /// Number of times to run
    count: usize
}

impl FractalArgs {
    fn run(&self) {
        if let Some(l_system) = parsing::parse_l_system(
            &fs::read_to_string(&self.input).expect("Something went wrong reading the file"),
        ) {
            save_program(&l_system.run(self.count), &self.output);
        } else {
            println!("L system could not be parsed");
        }
    }
}

fn main() {
    let command = Command::parse();
    command.run();
}
