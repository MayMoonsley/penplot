use std::collections::HashMap;
use nom::IResult;
use nom::{branch, bytes::complete::{tag_no_case, take_while}, character::complete, combinator, multi, sequence};
use crate::color::Color;
use crate::instruction::Instruction;
use crate::l_system::LSystem;

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

fn parse_address<'a>(symbol_table: Option<&'a HashMap<String, usize>>) -> impl FnMut(&'a str) -> IResult<&'a str, usize> {
    branch::alt((
        combinator::map(complete::u32, |x| x as usize), // a literal usize value
        combinator::map_opt(complete::alpha1, move |tag| {
            if let Some(symbol_table) = symbol_table {
                symbol_table.get(tag).copied()
            } else {
                None
            }
        }) // a label
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

pub fn parse_instruction<'a>(symbol_table: Option<&'a HashMap<String, usize>>, input: &'a str) -> IResult<&'a str, Instruction> {
    branch::alt((
        instruction_word("NOOP", |_| Instruction::Noop), // no-op
        instruction_word("RTRN", |_| Instruction::Return), // return
        instruction_word("BLOT", |_| Instruction::Blot), // blot
        instruction_word("HALT", |_| Instruction::Halt), // halt
        instruction_word("BLNK", |_| Instruction::SetColor(Color(0, 0, 0, 0))), // blank
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
            Instruction::MoveForward
        ), // move relative
        instruction_args("FACE",
            parse_isize_value,
            Instruction::Face
        ), // face
        instruction_args("TURN",
            parse_isize_value,
            Instruction::Turn
        ), // face
        instruction_args("GOTO",
            parse_address(symbol_table),
            Instruction::Goto
        ), // goto
        instruction_args("CALL",
            parse_address(symbol_table),
            Instruction::Call
        ), // call
        instruction_args("JUMP",
            parse_isize_value,
            Instruction::Jump
        ), // jump
        instruction_args("LOOP",
            sequence::separated_pair(parse_address(symbol_table), complete::space1, parse_usize_value),
            |(addr, num)| Instruction::Repeat(addr, num)
        ), // loop
        combinator::map(
            sequence::preceded(complete::char(';'), take_while(is_valid_comment_char)),
            |s: &str| Instruction::Comment(s.trim().to_string())
        ), // comment
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
        combinator::map(
            sequence::delimited(complete::char('<'), complete::anychar, complete::char('>')),
            |c| Instruction::Comment(c.to_string())
        ) // comment (single-char)
    ))(input)
}

fn parse_instruction_symless(input: &str) -> IResult<&str, Instruction> {
    parse_instruction(None, input)
}

// TODO: these need to return proper errors
pub fn parse_program(text: String) -> Option<Vec<Instruction>> {
    let split: Vec<&str> = text.trim().split('\n').collect();
    // generate symbol table
    let mut symbol_table: HashMap<String, usize> = HashMap::new();
    for (index, line) in split.iter().enumerate() {
        let mut command = line.trim().split('@');
        if let Some(label) = command.nth(1) {
            symbol_table.insert(label.trim().to_string(), index);
        }
    }
    // parse instructions
    let mut program: Vec<Instruction> = vec![];
    for string in split {
        match parse_instruction(Some(&symbol_table), string) {
            Ok((_, inst)) => program.push(inst),
            Err(e) => {
                println!("Error parsing code {:?}", e);
                return None;
            }
        }
    }
    Some(program)
}

// this parses the big curly-brace delimited
fn parse_l_system_value(input: &str) -> IResult<&str, Vec<Instruction>> {
    sequence::delimited(
        sequence::pair(complete::char('{'), complete::multispace1),
        multi::many1(sequence::terminated(parse_instruction_symless, complete::multispace1)),
        sequence::delimited(complete::multispace0, complete::char('}'), complete::multispace0)
    )(input)
}

fn parse_seed(input: &str) -> IResult<&str, Vec<Instruction>> {
    sequence::delimited(
        sequence::pair(tag_no_case("seed"), complete::multispace1),
        parse_l_system_value,
        complete::multispace0
    )(input)
}

fn parse_aliases(input: &str) -> IResult<&str, HashMap<Instruction, Vec<Instruction>>> {
    sequence::delimited(
        sequence::pair(tag_no_case("aliases"), complete::multispace1),
        sequence::delimited(
            sequence::pair(complete::char('{'), complete::multispace1),
            multi::fold_many1(sequence::terminated(parse_rule, complete::multispace0), HashMap::new, |mut map, (inst, rule)| {
                map.insert(inst, rule);
                map
            }),
            sequence::delimited(complete::multispace0, complete::char('}'), complete::multispace0)
        ),
        complete::multispace0
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (Instruction, Vec<Instruction>)> {
    sequence::separated_pair(
        parse_instruction_symless,
        complete::multispace1,
        parse_l_system_value
    )(input)
}

pub fn parse_l_system(input: &str) -> IResult<&str, LSystem> {
    // get the parameters in sequence
    let (input, seed) = parse_seed(input)?;
    // there might be a cleaner way to do this, but the idea is to allow aliases to exist here, but accept if they don't
    let (input, aliases) = match parse_aliases(input) {
        Ok((input, aliases)) => (input, Some(aliases)),
        Err(_) => (input, None)
    };
    // then we parse the rules...
    let (input, rules) = multi::fold_many1(parse_rule, HashMap::new, |mut map, (inst, rule)| {
        map.insert(inst, rule);
        map
    })(input)?;
    // and then we're done
    Ok((input, LSystem { seed, rules, aliases }))
}