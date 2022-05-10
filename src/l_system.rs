use crate::instruction::Instruction;
use crate::util;
use std::collections::HashMap;

pub struct LSystem {
    pub seed: Vec<Instruction>,
    pub rules: HashMap<Instruction, Vec<Instruction>>,
    pub aliases: Option<HashMap<Instruction, Vec<Instruction>>>
}

impl LSystem {
    // advance the L system by one step
    fn advance(&self, input: Vec<Instruction>) -> Vec<Instruction> {
        util::replace(input, &self.rules)
    }

    pub fn run(&self, iters: usize) -> Vec<Instruction> {
        let mut acc = self.seed.clone();
        for _ in 0..iters {
            acc = self.advance(acc);
        }
        if let Some(aliases) = &self.aliases {
            util::replace(acc, aliases)
        } else {
            acc
        }
    }
}