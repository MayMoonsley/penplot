use std::cmp::Ordering;

#[derive(Copy, Clone)]
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
                new_alpha
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

enum Instruction {
    Move(f32, f32), // move to X, Y
    MoveRel(f32, f32), // move by dX, dY
    MoveForward(f32), // move forward by N
    Turn(f32), // change heading by dT
    SetColor(Color),
    Blot, // set current pixel to pen color
    Comment(String) // makes L-systems easier to implement
}

struct ProgramState {
    pen_x: f32,
    pen_y: f32,
    pen_color: Color,
    heading: f32,
    width: usize,
    height: usize,
    buffer: Vec<Color>
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
            buffer
        }
    }
    
    fn exec_instruction(&mut self, command: &Instruction) {
        match command {
            Instruction::Move(x, y) => self.move_pen(*x, *y),
            Instruction::MoveRel(dx, dy) => self.move_pen(self.pen_x + *dx, self.pen_y + *dy),
            Instruction::MoveForward(dist) => {
                let dx = *dist * self.heading.cos();
                let dy = *dist * self.heading.sin();
                self.move_pen(self.pen_x + dx, self.pen_y + dy);
            }
            Instruction::Turn(theta) => self.heading += *theta,
            Instruction::SetColor(color) => self.pen_color = *color,
            Instruction::Blot => self.draw_pixel_f(self.pen_x, self.pen_y),
            Instruction::Comment(_) => ()
        };
    }
    
    fn move_pen(&mut self, new_x: f32, new_y: f32) {
        self.plot_line(self.pen_x.round() as isize, self.pen_y.round() as isize,
            new_x.round() as isize, new_y.round() as isize);
        self.pen_x = new_x;
        self.pen_y = new_y;
    }
    
    fn plot_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize) {
        let dx = (x1 - x0).abs();
        let sx = match x0.cmp(&x1) {
            Ordering::Less => 1,
            _ => -1
        };
        let dy = -(y1 - y0).abs();
        let sy = match y0.cmp(&y1) {
            Ordering::Less => 1,
            _ => -1
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
        image::save_buffer(filename, &bytes,
            self.width as u32, self.height as u32,
            image::ColorType::Rgba8).unwrap();
    }
}

fn main() {
    let mut program = ProgramState::new(512, 512);
    program.exec_instruction(&Instruction::Move(256.0, 256.0));
    program.exec_instruction(&Instruction::SetColor(Color(1.0, 1.0, 1.0, 1.0)));
    let points = 5;
    let dt = 3.14159 / (points as f32 * 0.25);
    for _ in 0..points {
        program.exec_instruction(&Instruction::MoveForward(64.0));
        program.exec_instruction(&Instruction::Turn(dt));
    }
    program.save_buffer("test.png");
}
