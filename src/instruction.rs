use crate::color::Color;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Instruction {
    Noop,                   // do nothing
    Move(f32, f32),         // move to X, Y
    MoveRel(f32, f32),      // move by dX, dY
    MoveForward(f32),       // move forward by N
    Face(f32),              // set heading to T
    Turn(f32),              // change heading by dT
    SetColor(Color),        // set pen color to c
    Blot,                   // set current pixel to pen color
    Comment(String),        // makes L-systems easier to implement
    Goto(usize),            // set pc to i
    Jump(isize),            // set pc to pc + i + 1
    Call(usize),            // call subroutine at position i
    Return,                 // return from subroutine call
    Repeat(usize, usize),   // repeat subroutine at position i n times
    Halt,                   // halt
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Instruction::Noop => write!(f, "NOOP"),
            Instruction::Move(x, y) => write!(f, "MOVE {} {}", x, y),
            Instruction::MoveRel(dx, dy) => write!(f, "SHFT {} {}", dx, dy),
            Instruction::MoveForward(n) => write!(f, "WALK {}", n),
            Instruction::Face(theta) => write!(f, "FACE {}", theta),
            Instruction::Turn(dt) => write!(f, "TURN {}", dt),
            Instruction::SetColor(color) => write!(f, "RGBA {}", color),
            Instruction::Blot => write!(f, "BLOT"),
            Instruction::Comment(s) => write!(f, "; {}", s),
            Instruction::Goto(i) => write!(f, "GOTO {}", i),
            Instruction::Jump(i) => write!(f, "JUMP {}", i),
            Instruction::Call(i) => write!(f, "CALL {}", i),
            Instruction::Return => write!(f, "RTRN"),
            Instruction::Repeat(i, n) => write!(f, "LOOP {} {}", i, n),
            Instruction::Halt => write!(f, "HALT"),
        }
    }
}

impl Instruction {
    fn token_to_address(text: &str, symbol_table: &HashMap<String, usize>) -> Option<usize> {
        match symbol_table.get(text) {
            Some(add) => Some(*add),
            None => text.parse().ok(),
        }
    }

    // TODO: return errors
    fn from_string(text: &str, symbol_table: &HashMap<String, usize>) -> Option<Instruction> {
        // TODO: use proper parser combinators and not this nasty mess
        let mut split = text.trim().split(' ');
        match split.next()? {
            "NOOP" => Some(Instruction::Noop),
            "MOVE" => Some(Instruction::Move(
                split.next()?.parse().ok()?,
                split.next()?.parse().ok()?,
            )),
            "SHFT" => Some(Instruction::MoveRel(
                split.next()?.parse().ok()?,
                split.next()?.parse().ok()?,
            )),
            "WALK" => Some(Instruction::MoveForward(split.next()?.parse().ok()?)),
            "FACE" => Some(Instruction::Face(split.next()?.parse().ok()?)),
            "FCE%" => {
                let theta: f32 = split.next()?.parse().ok()?;
                Some(Instruction::Face(theta * TAU))
            }
            "TURN" => Some(Instruction::Turn(split.next()?.parse().ok()?)),
            "TRN%" => {
                let theta: f32 = split.next()?.parse().ok()?;
                Some(Instruction::Turn(theta * TAU))
            }
            "RGBA" => {
                let first = split.next()?;
                match first.parse::<Color>() {
                    Ok(color) => Some(Instruction::SetColor(color)),
                    Err(_) => Some(Instruction::SetColor(Color(
                        first.parse().ok()?,
                        split.next()?.parse().ok()?,
                        split.next()?.parse().ok()?,
                        split.next()?.parse().ok()?,
                    ))),
                }
            }
            "BLNK" => Some(Instruction::SetColor(Color(0.0, 0.0, 0.0, 0.0))),
            "BLOT" => Some(Instruction::Blot),
            ";" => Some(Instruction::Comment(split.as_str().to_string())),
            "GOTO" => Some(Instruction::Goto(Instruction::token_to_address(
                split.next()?,
                symbol_table,
            )?)),
            "JUMP" => Some(Instruction::Jump(split.next()?.parse().ok()?)),
            "CALL" => Some(Instruction::Call(split.next()?.parse().ok()?)),
            "RTRN" => Some(Instruction::Return),
            "LOOP" => Some(Instruction::Repeat(
                Instruction::token_to_address(split.next()?, symbol_table)?,
                split.next()?.parse().ok()?,
            )),
            "HALT" => Some(Instruction::Halt),
            _ => None,
        }
    }

    pub fn parse_program(text: String) -> Option<Vec<Instruction>> {
        let split: Vec<&str> = text.trim().split('\n').collect();
        // generate symbol table
        let mut symbol_table: HashMap<String, usize> = HashMap::new();
        for i in 0..split.len() {
            let mut command = split[i].split('@');
            if let Some(label) = command.nth(1) {
                symbol_table.insert(label.trim().to_string(), i);
            }
        }
        // parse instructions
        let mut program: Vec<Instruction> = vec![];
        for string in split {
            program.push(Instruction::from_string(string, &symbol_table)?);
        }
        Some(program)
    }
}
