#![feature(str_split_as_str)]
use std::cmp::Ordering;
use std::f32::consts::TAU;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Color(f32, f32, f32, f32); // RGBA in [0, 1]

impl Color {
    fn overlay(top: Color, bottom: Color) -> Color {
        if top.alpha() == 0.0 && bottom.alpha() == 0.0 {
            // avoid division by zero errors
            Color(0.0, 0.0, 0.0, 0.0)
        } else {
            let inv_alpha = 1.0 - top.alpha();
            let new_alpha = top.alpha() + bottom.alpha() * inv_alpha;
            let scale = 1.0 / new_alpha;
            Color(
                (top.red() * top.alpha() + bottom.red() * bottom.alpha() * inv_alpha) * scale,
                (top.green() * top.alpha() + bottom.green() * bottom.alpha() * inv_alpha) * scale,
                (top.blue() * top.alpha() + bottom.blue() * bottom.alpha() * inv_alpha) * scale,
                new_alpha,
            )
        }
    }

    #[inline]
    fn red(&self) -> f32 {
        self.0
    }

    #[inline]
    fn green(&self) -> f32 {
        self.1
    }

    #[inline]
    fn blue(&self) -> f32 {
        self.2
    }

    #[inline]
    fn alpha(&self) -> f32 {
        self.3
    }

    #[inline]
    fn red_byte(&self) -> u8 {
        (self.0 * 255.0) as u8
    }

    #[inline]
    fn green_byte(&self) -> u8 {
        (self.1 * 255.0) as u8
    }

    #[inline]
    fn blue_byte(&self) -> u8 {
        (self.2 * 255.0) as u8
    }

    #[inline]
    fn alpha_byte(&self) -> u8 {
        (self.3 * 255.0) as u8
    }
}

#[derive(Debug)]
enum ColorParseError {
    NoHash,
    BadInt,
    WrongLength,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorParseError::NoHash => write!(f, "missing #"),
            ColorParseError::BadInt => write!(f, "malformed int"),
            ColorParseError::WrongLength => write!(f, "wrong length"),
        }
    }
}

impl From<ParseIntError> for ColorParseError {
    fn from(_err: ParseIntError) -> Self {
        ColorParseError::BadInt
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        if hex_code.len() != 7 && hex_code.len() != 9 {
            Err(ColorParseError::WrongLength)
        } else if &hex_code[0..1] != "#" {
            Err(ColorParseError::NoHash)
        } else {
            let r: f32 = u8::from_str_radix(&hex_code[1..3], 16)? as f32;
            let g: f32 = u8::from_str_radix(&hex_code[3..5], 16)? as f32;
            let b: f32 = u8::from_str_radix(&hex_code[5..7], 16)? as f32;
            if hex_code.len() == 9 {
                let a: f32 = u8::from_str_radix(&hex_code[7..9], 16)? as f32;
                Ok(Color(r / 255.0, g / 255.0, b / 255.0, a / 255.0))
            } else {
                Ok(Color(r / 255.0, g / 255.0, b / 255.0, 1.0))
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
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
    fn from_string(text: &str) -> Option<Instruction> {
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

    fn parse_program(text: String) -> Vec<Instruction> {
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

struct ProgramState {
    pen_x: f32,
    pen_y: f32,
    pen_color: Color,
    heading: f32,
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    program_counter: usize,
    call_stack: Vec<usize>,
}

impl ProgramState {
    fn new(width: usize, height: usize) -> ProgramState {
        let buffer: Vec<Color> = vec![Color(0.0, 0.0, 0.0, 0.0); width * height];
        ProgramState {
            pen_x: 0.0,
            pen_y: 0.0,
            heading: 0.0,
            pen_color: Color(0.0, 0.0, 0.0, 0.0),
            width,
            height,
            buffer,
            program_counter: 0,
            call_stack: vec![],
        }
    }

    fn execute(&mut self, commands: Vec<Instruction>) {
        self.program_counter = 0;
        loop {
            self.program_counter = match commands.get(self.program_counter) {
                Some(command) => self.exec_instruction(&command),
                None => break,
            }
        }
    }

    // returns new program counter
    fn exec_instruction(&mut self, command: &Instruction) -> usize {
        let new_pc: Option<usize> = match command {
            Instruction::Move(x, y) => {
                self.move_pen(*x, *y);
                None
            }
            Instruction::MoveRel(dx, dy) => {
                self.move_pen(self.pen_x + *dx, self.pen_y + *dy);
                None
            }
            Instruction::MoveForward(dist) => {
                let dx = *dist * self.heading.cos();
                let dy = *dist * self.heading.sin();
                self.move_pen(self.pen_x + dx, self.pen_y + dy);
                None
            }
            Instruction::Face(theta) => {
                self.heading = *theta;
                None
            }
            Instruction::Turn(theta) => {
                self.heading += *theta;
                None
            }
            Instruction::SetColor(color) => {
                self.pen_color = *color;
                None
            }
            Instruction::Blot => {
                self.draw_pixel_f(self.pen_x, self.pen_y);
                None
            }
            Instruction::Comment(_) => None,
            Instruction::Goto(pc) => Some(*pc),
            Instruction::Jump(i) => {
                let new_pc = self.program_counter as isize + *i + 1;
                if new_pc < 0 {
                    Some(0)
                } else {
                    Some(new_pc as usize)
                }
            }
            Instruction::Call(pc) => {
                self.call_stack.push(self.program_counter + 1);
                Some(*pc)
            }
            Instruction::Return => self.call_stack.pop(),
            Instruction::Repeat(pc, n) => {
                let pc = *pc;
                self.call_stack.push(self.program_counter + 1);
                for _ in 0..(*n - 1) {
                    self.call_stack.push(pc);
                }
                Some(pc)
            }
        };
        match new_pc {
            None => self.program_counter + 1,
            Some(pc) => pc,
        }
    }

    fn move_pen(&mut self, new_x: f32, new_y: f32) {
        self.plot_line(
            self.pen_x.round() as isize,
            self.pen_y.round() as isize,
            new_x.round() as isize,
            new_y.round() as isize,
        );
        self.pen_x = new_x;
        self.pen_y = new_y;
    }

    fn plot_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize) {
        let dx = (x1 - x0).abs();
        let sx = match x0.cmp(&x1) {
            Ordering::Less => 1,
            _ => -1,
        };
        let dy = -(y1 - y0).abs();
        let sy = match y0.cmp(&y1) {
            Ordering::Less => 1,
            _ => -1,
        };
        let mut err = dx + dy;
        loop {
            self.draw_pixel_i(x0, y0);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_pixel_f(&mut self, x: f32, y: f32) {
        self.draw_pixel_i(x.round() as isize, y.round() as isize);
    }

    fn draw_pixel_i(&mut self, x: isize, y: isize) {
        let w = self.width as isize;
        let h = self.height as isize;
        if x < 0 || y < 0 || x >= w || y >= h {
            return;
        } else {
            let index = (x + y * w) as usize;
            self.buffer[index] = Color::overlay(self.pen_color, self.buffer[index]);
        }
    }

    fn save_buffer(&self, filename: &str) {
        let mut bytes: Vec<u8> = vec![0; self.width * self.height * 4];
        for index in 0..self.width * self.height {
            bytes[index * 4] = self.buffer[index].red_byte();
            bytes[index * 4 + 1] = self.buffer[index].green_byte();
            bytes[index * 4 + 2] = self.buffer[index].blue_byte();
            bytes[index * 4 + 3] = self.buffer[index].alpha_byte();
        }
        image::save_buffer(
            filename,
            &bytes,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }
}

fn main() {
    let mut program = ProgramState::new(512, 512);
    let commands = Instruction::parse_program(
        fs::read_to_string("test.txt").expect("Something went wrong reading the file"),
    );
    program.execute(commands);
    program.save_buffer("test.png");
}
