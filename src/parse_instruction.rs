use std::collections::HashMap;
use nom::IResult;
use nom::{branch, bytes::complete::{tag_no_case, take_while}, character, character::complete, combinator, sequence};
use crate::instruction::Instruction;

// predicate for if a char can go in a comment
// this is every char except line ending chars (/r, /n) and the labeling char (@)
fn is_valid_comment_char(c: char) -> bool {
    !(c == '@' || c == '\n' || c == '\r')
}

// This function is its own thing so it can eventually accommodate registers
fn parse_isize_value(input: &str) -> IResult<&str, isize> {
    complete::i32(input).map(|(x, y)| (x, y as isize))
}

fn parse_address<'a>(symbol_table: &'a HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, usize> {
    branch::alt((
        combinator::map(complete::u32, |x| x as usize), // a literal usize value
        combinator::map_opt(complete::alpha1, move |tag| symbol_table.get(tag).map(|&x| x)) // a label
    ))
}

// TODO: there's gotta be a way to avoid all this code repetition
pub fn parse_instruction<'a>(symbol_table: &'a HashMap<String, usize>, input: &'a str) -> IResult<&'a str, Instruction> {
    branch::alt((
        combinator::map(tag_no_case("NOOP"), |_| Instruction::Noop), // no-op
        combinator::map(tag_no_case("RTRN"), |_| Instruction::Return), // return
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("MOVE"), complete::space1),
                sequence::separated_pair(parse_isize_value, complete::space1, parse_isize_value)
            ),
            |(x, y)| Instruction::Move(x, y)
        ), // move
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("SHFT"), complete::space1),
                sequence::separated_pair(parse_isize_value, complete::space1, parse_isize_value)
            ),
            |(dx, dy)| Instruction::MoveRel(dx, dy)
        ), // move relative
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("WALK"), complete::space1),
                parse_isize_value
            ),
            |len| Instruction::MoveForward(len)
        ), // move forward
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("FACE"), complete::space1),
                parse_isize_value
            ),
            |theta| Instruction::Face(theta)
        ), // face
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("TURN"), complete::space1),
                parse_isize_value
            ),
            |theta| Instruction::Turn(theta)
        ), // turn
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("GOTO"), complete::space1),
                parse_address(symbol_table)
            ),
            |addr| Instruction::Goto(addr)
        ), // goto
        combinator::map(
            sequence::preceded(
                sequence::pair(tag_no_case("CALL"), complete::space1),
                parse_address(symbol_table)
            ),
            |addr| Instruction::Call(addr)
        ), // call
        combinator::map(
            sequence::preceded(complete::char(';'), take_while(is_valid_comment_char)),
            |s: &str| Instruction::Comment(s.to_string())
        ), // comment
    ))(input)
}