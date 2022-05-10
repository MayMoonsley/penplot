use crate::color::Color;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Instruction {
    Noop,                   // do nothing
    Move(isize, isize),         // move to X, Y
    MoveRel(isize, isize),      // move by dX, dY
    MoveForward(isize),       // move forward by N
    Face(isize),              // set heading to T
    Turn(isize),              // change heading by dT
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
