#![feature(str_split_as_str)]
mod color;
mod instruction;
mod program_state;

use std::fs;
use crate::instruction::Instruction;
use crate::program_state::ProgramState;

fn main() {
    let mut program = ProgramState::new(512, 512);
    let commands = Instruction::parse_program(
        fs::read_to_string("test.txt").expect("Something went wrong reading the file"),
    );
    program.execute(commands);
    program.save_buffer("test.png");
}
