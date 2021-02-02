use crate::instruction::Instruction;

pub trait LSystem {
    fn seed(&self) -> Vec<Instruction>;

    fn rule(&self, inst: &Instruction) -> Option<Vec<Instruction>>;

    fn evaluate(&self, instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut result: Vec<Instruction> = vec![];
        for inst in instructions {
            match self.rule(&inst) {
                Some(vec) => {
                    for i in vec {
                        result.push(i);
                    }
                }
                None => result.push(inst),
            }
        }
        result
    }

    fn iterate(&self, count: usize) -> Vec<Instruction> {
        let mut acc = self.seed();
        for _ in 0..count {
            acc = self.evaluate(acc);
        }
        acc
    }
}

pub enum Fractal {
    Koch(f32, f32),
}

impl LSystem for Fractal {
    fn seed(&self) -> Vec<Instruction> {
        match self {
            Fractal::Koch(len, _) => vec![Instruction::MoveForward(*len)],
        }
    }

    fn rule(&self, inst: &Instruction) -> Option<Vec<Instruction>> {
        match self {
            Fractal::Koch(len, angle) => match inst {
                Instruction::MoveForward(_) => Some(vec![
                    Instruction::MoveForward(*len),
                    Instruction::Turn(-*angle),
                    Instruction::MoveForward(*len),
                    Instruction::Turn(*angle),
                    Instruction::MoveForward(*len),
                    Instruction::Turn(*angle),
                    Instruction::MoveForward(*len),
                    Instruction::Turn(-*angle),
                    Instruction::MoveForward(*len),
                ]),
                _ => None,
            },
        }
    }
}
