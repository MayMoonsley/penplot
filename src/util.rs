use std::collections::HashMap;
use std::hash::Hash;

// Do replacement on a Vec, given a HashMap of replacement rules
pub fn replace<T: Clone + Eq + Hash>(input: Vec<T>, rules: &HashMap<T, Vec<T>>) -> Vec<T> {
    let mut result = vec![];
    for item in input.into_iter() {
        if let Some(rule) = rules.get(&item) {
            result.extend(rule.clone());
        } else {
            result.push(item);
        }
    }
    result
}