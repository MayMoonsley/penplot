#![feature(str_split_as_str)]
mod color;
mod instruction;
mod program_state;

use crate::instruction::Instruction;
use crate::program_state::ProgramState;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = match args.get(1) {
        Some(s) => s,
        None => {
            println!("Error: program filename required");
            std::process::exit(1);
        }
    };
    let out = match args.get(2) {
        Some(s) => s,
        None => {
            println!("Error: output filename required");
            std::process::exit(1);
        }
    };
    let mut program = ProgramState::new(512, 512);
    let commands = Instruction::parse_program(
        fs::read_to_string(&source).expect("Something went wrong reading the file"),
    );
    program.execute(commands.expect("Erorr parsing code"));
    program.save_buffer(&out);
}
