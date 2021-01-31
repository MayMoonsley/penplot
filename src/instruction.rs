use std::f32::consts::TAU;
use crate::color::Color;

#[derive(Debug)]
pub enum Instruction {
    Move(f32, f32),    // move to X, Y
    MoveRel(f32, f32), // move by dX, dY
    MoveForward(f32),  // move forward by N
    Face(f32),         // set heading to T
    Turn(f32),         // change heading by dT
    SetColor(Color),
    Blot,                 // set current pixel to pen color
    Comment(String),      // makes L-systems easier to implement
    Goto(usize),          // set pc to i
    Jump(isize),          // set pc to pc + i + 1
    Call(usize),          // call subroutine at position i
    Return,               // return from subroutine call
    Repeat(usize, usize), // repeat subroutine at position i n times
}

impl Instruction {
    // TODO: use the proper FromStr method
    pub fn from_string(text: &str) -> Option<Instruction> {
        // TODO: use proper parser combinators and not this nasty mess
        let mut split = text.split(' ');
        match split.next()? {
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
            "GOTO" => Some(Instruction::Goto(split.next()?.parse().ok()?)),
            "JUMP" => Some(Instruction::Jump(split.next()?.parse().ok()?)),
            "CALL" => Some(Instruction::Call(split.next()?.parse().ok()?)),
            "RTRN" => Some(Instruction::Return),
            "LOOP" => Some(Instruction::Repeat(
                split.next()?.parse().ok()?,
                split.next()?.parse().ok()?,
            )),
            _ => None,
        }
    }

    pub fn parse_program(text: String) -> Vec<Instruction> {
        let split = text.split('\n');
        let mut program: Vec<Instruction> = vec![];
        for string in split {
            match Instruction::from_string(string) {
                Some(command) => program.push(command),
                None => (),
            }
        }
        program
    }
}

