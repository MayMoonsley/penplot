#![feature(stdin_forwarders)]
mod color;
mod instruction;
mod l_system;
mod parsing;
mod program_state;
mod util;

use crate::instruction::Instruction;
use crate::program_state::ProgramState;
use std::fs::{self, File};
use std::io::Result as IoResult;
use std::io::{self, Write};
use clap::{Args, Parser, Subcommand};

fn save_program(code: &[Instruction], filename: &str) -> IoResult<()> {
    let mut buffer = File::create(filename)?;
    for line in code {
        writeln!(buffer, "{}", line)?;
    }
    Ok(())
}

fn read_stdin_to_string() -> String {
    let mut acc = String::new();
    for line in io::stdin().lines() {
        acc.push_str(&line.unwrap()); // append the line
        acc.push('\n'); // then append a newline (since lines() removes them)
    }
    acc
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
    /// Filename of source code to run (if omitted, use stdin)
    #[clap(short, long)]
    input: Option<String>,
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
        let source_code = if let Some(filename) = &self.input {
            fs::read_to_string(filename).expect("Something went wrong reading the file")
        } else {
            read_stdin_to_string()
        };
        let commands = parsing::parse_program(source_code);
        program.execute(commands.expect("Error parsing code"));
        program.save_buffer(&self.output);
    }
}

/// Iterate a specified L system and save its code output to a file
#[derive(Args)]
struct FractalArgs {
    /// Filename of L system specification (if omitted, use stdin)
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long)]
    /// Filename to save the resulting code as (if omitted, use stdout)
    output: Option<String>,
    #[clap(short, long)]
    /// Number of times to run
    count: usize
}

impl FractalArgs {
    fn run(&self) {
        let system_spec = if let Some(filename) = &self.input {
            fs::read_to_string(filename).expect("Something went wrong reading the file")
        } else {
            read_stdin_to_string()
        };
        match parsing::parse_l_system(&system_spec) {
            Ok((_, l_system)) => {
                let program = l_system.run(self.count);
                if let Some(filename) = &self.output {
                    save_program(&program, filename).expect("Error saving program");
                } else {
                    for inst in program {
                        println!("{}", inst);
                    }
                }
            }
            Err(e) => println!("L system could not be parsed (error {:?})", e)
        }
    }
}

fn main() {
    let command = Command::parse();
    command.run();
}
