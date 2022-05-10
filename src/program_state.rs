use crate::color::Color;
use crate::instruction::Instruction;
use std::cmp::Ordering;

pub struct ProgramState {
    pen_x: f32,
    pen_y: f32,
    pen_color: Color,
    heading: f32,
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    program_counter: usize,
    executing: bool,
    call_stack: Vec<usize>,
}

impl ProgramState {
    pub fn new(width: usize, height: usize) -> ProgramState {
        let buffer: Vec<Color> = vec![Color::transparent(); width * height];
        ProgramState {
            pen_x: 0.0,
            pen_y: 0.0,
            heading: 0.0,
            pen_color: Color(0, 0, 0, 255),
            width,
            height,
            buffer,
            program_counter: 0,
            executing: true,
            call_stack: vec![],
        }
    }

    pub fn execute(&mut self, commands: Vec<Instruction>) {
        self.program_counter = 0;
        self.executing = true;
        while self.executing {
            self.program_counter = match commands.get(self.program_counter) {
                Some(command) => self.exec_instruction(command),
                None => break,
            }
        }
    }

    // returns new program counter
    fn exec_instruction(&mut self, command: &Instruction) -> usize {
        let new_pc: Option<usize> = match command {
            Instruction::Noop => None,
            Instruction::Move(x, y) => {
                self.move_pen(*x as f32, *y as f32);
                None
            }
            Instruction::MoveRel(dx, dy) => {
                self.move_pen(self.pen_x + *dx as f32, self.pen_y + *dy as f32);
                None
            }
            Instruction::MoveForward(dist) => {
                let dist = *dist as f32;
                let dx = dist * self.heading.cos();
                let dy = dist * self.heading.sin();
                self.move_pen(self.pen_x + dx, self.pen_y + dy);
                None
            }
            Instruction::Face(theta) => {
                self.heading = (*theta as f32).to_radians();
                None
            }
            Instruction::Turn(theta) => {
                self.heading += (*theta as f32).to_radians();
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
            Instruction::Halt => {
                self.executing = false;
                None
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

        } else {
            let index = (x + y * w) as usize;
            self.buffer[index] = Color::overlay(self.pen_color, self.buffer[index]);
        }
    }

    pub fn save_buffer(&self, filename: &str) {
        let mut bytes: Vec<u8> = vec![0; self.width * self.height * 4];
        for index in 0..self.width * self.height {
            let Color(r, g, b, a) = self.buffer[index];
            bytes[index * 4] = r;
            bytes[index * 4 + 1] = g;
            bytes[index * 4 + 2] = b;
            bytes[index * 4 + 3] = a;
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
