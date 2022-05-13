use crate::canvas::DrawingCanvas;
use crate::instruction::Instruction;

pub struct ProgramState<T: DrawingCanvas> {
    pen_x: f32,
    pen_y: f32,
    heading: f32,
    canvas: T,
    program_counter: usize,
    executing: bool,
    call_stack: Vec<usize>,
}

impl<T: DrawingCanvas> ProgramState<T> {
    pub fn new(canvas: T) -> ProgramState<T> {
        ProgramState {
            canvas,
            pen_x: 0.0,
            pen_y: 0.0,
            heading: 0.0,
            program_counter: 0,
            executing: true,
            call_stack: vec![],
        }
    }

    pub fn execute(&mut self, commands: &[Instruction]) {
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
                let (x, y) = (*x as f32, *y as f32);
                self.canvas.move_pen_to(x, y);
                self.pen_x = x;
                self.pen_y = y;
                None
            }
            Instruction::MoveRel(dx, dy) => {
                let (dx, dy) = (*dx as f32, *dy as f32);
                self.canvas.move_pen_to(self.pen_x + dx, self.pen_y + dy);
                self.pen_x += dx;
                self.pen_y += dy;
                None
            }
            Instruction::MoveForward(dist) => {
                let dist = *dist as f32;
                let dx = dist * self.heading.cos();
                let dy = dist * self.heading.sin();
                self.canvas.move_pen_to(self.pen_x + dx, self.pen_y + dy);
                self.pen_x += dx;
                self.pen_y += dy;
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
                self.canvas.set_color(*color);
                None
            }
            Instruction::Blot => {
                self.canvas.blot(self.pen_x, self.pen_y);
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

    pub fn save_canvas(&self, filename: &str) {
        self.canvas.save(filename);
    }
}
