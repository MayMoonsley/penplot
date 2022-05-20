mod canvas;
mod color;
mod instruction;
mod l_system;
mod parsing;
mod program_state;
mod util;

use crate::canvas::{PixelCanvas, SizingCanvas};
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
    #[clap(long)]
    width: Option<usize>,
    /// Height of canvas
    #[clap(long)]
    height: Option<usize>
}

impl RunArgs {
    fn run(&self) {
        // load program
        let source_code = if let Some(filename) = &self.input {
            fs::read_to_string(filename).expect("Something went wrong reading the file")
        } else {
            read_stdin_to_string()
        };
        let commands = parsing::parse_program(source_code).expect("Error parsing code");
        // determine size + offset
        let (width, height, x_offset, y_offset) = if let Some((width, height)) = self.width.zip(self.height) {
            (width, height, 0, 0)
        } else {
            let sizing_canvas = SizingCanvas::new();
            let mut sizing_program = ProgramState::new(sizing_canvas);
            sizing_program.execute(&commands);
            // since the program took ownership of the sizing canvas, we need to get it back
            let sizing_canvas = sizing_program.canvas();
            let (width, height) = sizing_canvas.dimensions();
            let (x_offset, y_offset) = sizing_canvas.offsets();
            (width, height, x_offset, y_offset)
        };
        let canvas = PixelCanvas::new(width, height, x_offset, y_offset);
        let mut program = ProgramState::new(canvas);
        program.execute(&commands);
        program.save_canvas(&self.output);
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
