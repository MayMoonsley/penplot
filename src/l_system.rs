use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct LSystem {
    pub seed: Vec<Instruction>,
    pub rules: HashMap<Instruction, Vec<Instruction>>
}

impl LSystem {

}