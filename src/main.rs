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

#[derive(Copy, Clone)]
enum Instruction {
    Move(usize, usize), // move to X, Y
    SetColor(Color),
    Blot, // set current pixel to pen color
    Comment(&'static str) // makes L-systems easier to implement
}

struct ProgramState {
    pen_x: usize,
    pen_y: usize,
    pen_color: Color,
    width: usize,
    height: usize,
    buffer: Vec<Color>
}

impl ProgramState {
    fn new(width: usize, height: usize) -> ProgramState {
        let buffer: Vec<Color> = vec![Color(0.0, 0.0, 0.0, 0.0); width * height];
        ProgramState {
            pen_x: 0,
            pen_y: 0,
            pen_color: Color(0.0, 0.0, 0.0, 0.0),
            width,
            height,
            buffer
        }
    }
    
    fn exec_instruction(&mut self, command: Instruction) {
        match command {
            Instruction::Move(x, y) => self.move_pen(x, y),
            Instruction::SetColor(color) => self.pen_color = color,
            Instruction::Blot => self.draw_pixel(self.pen_x, self.pen_y),
            Instruction::Comment(_) => ()
        };
    }
    
    fn move_pen(&mut self, x: usize, y: usize) {
        let new_x = x % self.width;
        let new_y = y % self.height;
        // TODO: draw line from old to new positions
        self.pen_x = new_x;
        self.pen_y = new_y;
    }
    
    fn draw_pixel(&mut self, x: usize, y: usize) {
        let index = x + y * self.width;
        self.buffer[index] = Color::overlay(self.pen_color, self.buffer[index]);
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
    program.exec_instruction(Instruction::SetColor(Color(0.0, 1.0, 1.0, 1.0)));
    for i in 0..64 {
        program.exec_instruction(Instruction::Blot);
        program.exec_instruction(Instruction::Move(i * 8, i * 8));
    }
    program.save_buffer("test.png");
}
