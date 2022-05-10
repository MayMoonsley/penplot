use std::collections::HashMap;
use nom::IResult;
use nom::{branch, bytes::complete::{tag_no_case, take_while}, character, character::complete, combinator, sequence};
use crate::color::Color;
use crate::instruction::Instruction;

// predicate for if a char can go in a comment
// this is every char except line ending chars (/r, /n) and the labeling char (@)
fn is_valid_comment_char(c: char) -> bool {
    !(c == '@' || c == '\n' || c == '\r')
}

// These function are their own thing so they can eventually accommodate registers
fn parse_usize_value(input: &str) -> IResult<&str, usize> {
    complete::u32(input).map(|(x, y)| (x, y as usize))
}

fn parse_isize_value(input: &str) -> IResult<&str, isize> {
    complete::i32(input).map(|(x, y)| (x, y as isize))
}

fn parse_address<'a>(symbol_table: &'a HashMap<String, usize>) -> impl FnMut(&'a str) -> IResult<&'a str, usize> {
    branch::alt((
        combinator::map(complete::u32, |x| x as usize), // a literal usize value
        combinator::map_opt(complete::alpha1, move |tag| symbol_table.get(tag).map(|&x| x)) // a label
    ))
}

fn instruction_args<'a, F, G, T>(name: &'static str, parser: F, mapper: G) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction>
where
    F: FnMut(&'a str) -> IResult<&'a str, T>,
    G: FnMut(T) -> Instruction
{
    combinator::map(
        sequence::preceded(
            sequence::pair(tag_no_case(name), complete::space1),
            parser
        ),
        mapper
    )
}

fn instruction_args_opt<'a, F, G, T>(name: &'static str, parser: F, mapper: G) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction>
where
    F: FnMut(&'a str) -> IResult<&'a str, T>,
    G: FnMut(T) -> Option<Instruction>
{
    combinator::map_opt(
        sequence::preceded(
            sequence::pair(tag_no_case(name), complete::space1),
            parser
        ),
        mapper
    )
}

// helper function for instructions with no params
fn instruction_word<'a, F: Fn(&'a str) -> Instruction>(name: &'static str, instruction: F) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction> {
    combinator::map(tag_no_case(name), instruction)
}

// TODO: there's gotta be a way to avoid all this code repetition
pub fn parse_instruction<'a>(symbol_table: &'a HashMap<String, usize>, input: &'a str) -> IResult<&'a str, Instruction> {
    branch::alt((
        instruction_word("NOOP", |_| Instruction::Noop), // no-op
        instruction_word("RTRN", |_| Instruction::Return), // return
        instruction_word("BLOT", |_| Instruction::Blot), // blot
        instruction_word("HALT", |_| Instruction::Halt), // halt
        instruction_word("BLNK", |_| Instruction::SetColor(Color(0.0, 0.0, 0.0, 0.0))), // blank
        instruction_args("MOVE",
            sequence::separated_pair(parse_isize_value, complete::space1, parse_isize_value),
            |(x, y)| Instruction::Move(x, y)
        ),
        instruction_args("SHFT",
            sequence::separated_pair(parse_isize_value, complete::space1, parse_isize_value),
            |(dx, dy)| Instruction::MoveRel(dx, dy)
        ), // move relative
        instruction_args("WALK",
            parse_isize_value,
            |len| Instruction::MoveForward(len)
        ), // move relative
        instruction_args("FACE",
            parse_isize_value,
            |theta| Instruction::Face(theta)
        ), // face
        instruction_args("TURN",
            parse_isize_value,
            |theta| Instruction::Turn(theta)
        ), // face
        instruction_args("GOTO",
            parse_address(symbol_table),
            |addr| Instruction::Goto(addr)
        ), // goto
        instruction_args("CALL",
            parse_address(symbol_table),
            |addr| Instruction::Call(addr)
        ), // call
        instruction_args("JUMP",
            parse_isize_value,
            |dist| Instruction::Jump(dist)
        ), // jump
        instruction_args("LOOP",
            sequence::separated_pair(parse_address(symbol_table), complete::space1, parse_usize_value),
            |(addr, num)| Instruction::Repeat(addr, num)
        ), // loop
        instruction_args(";",
            take_while(is_valid_comment_char),
            |s: &str| Instruction::Comment(s.trim().to_string())
        ), // comments
        instruction_args_opt("RGBA",
            sequence::separated_pair(
                sequence::separated_pair(parse_usize_value, complete::space1, parse_usize_value),
                complete::space1,
                sequence::separated_pair(parse_usize_value, complete::space1, parse_usize_value)
            ),
            |((r, g), (b, a))| Some(
                Instruction::SetColor(Color::from_ints(r, g, b, a)?)
            )
        ), // set color (RGBA)
        instruction_args_opt("RGB",
            sequence::separated_pair(
                sequence::separated_pair(parse_usize_value, complete::space1, parse_usize_value),
                complete::space1, parse_usize_value
            ),
            |((r, g), b)| Some(
                Instruction::SetColor(Color::from_ints(r, g, b, 255)?)
            )
        ), // set color (RGB)
    ))(input)
}