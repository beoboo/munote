use nom::{bytes::complete::take_while, character::complete::char, combinator::value, sequence::delimited, IResult};

pub mod accidentals;
pub mod chord;
pub mod context;
pub mod dots;
pub mod duration;
pub mod note;
pub mod rest;
pub mod score;
pub mod symbol;
pub mod tag;
pub mod voice;

fn ws(input: &str) -> IResult<&str, &str> {
    take_while(is_whitespace)(input)
}

fn comma(input: &str) -> IResult<&str, ()> {
    value((), delimited(ws, char(','), ws))(input)
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}
