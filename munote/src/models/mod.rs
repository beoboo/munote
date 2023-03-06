use std::fmt::Debug;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::combinator::value;
use nom::sequence::delimited;
use nom::IResult;
use crate::duration::Duration;

pub mod accidentals;
pub mod chord;
pub mod dots;
pub mod duration;
pub mod note;
pub mod rest;
pub mod voice;
pub mod symbol;
pub mod context;

fn ws(input: &str) -> IResult<&str, &str> {
    take_while(is_whitespace)(input)
}

fn comma(input: &str) -> IResult<&str, ()> {
    value((), delimited(ws, char(','), ws))(input)
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}
