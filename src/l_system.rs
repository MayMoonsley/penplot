use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct LSystem {
    pub seed: Vec<Instruction>,
    pub rules: HashMap<Instruction, Vec<Instruction>>
}

impl LSystem {
    // advance the L system by one step
    fn advance(&self, input: Vec<Instruction>) -> Vec<Instruction> {
        let mut result = vec![];
        for inst in input.into_iter() {
            if let Some(rule) = self.rules.get(&inst) {
                result.extend(rule.clone());
            } else {
                result.push(inst);
            }
        }
        result
    }

    pub fn run(&self, iters: usize) -> Vec<Instruction> {
        let mut acc = self.seed.clone();
        for _ in 0..iters {
            acc = self.advance(acc);
        }
        acc
    }
}