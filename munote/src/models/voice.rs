use std::convert::From;
use std::str::FromStr;

use nom::character::complete::{char, one_of};
use nom::combinator::peek;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

use crate::duration::Duration;
use crate::models::ws;
use crate::note::Note;
use crate::rest::Rest;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Voice {
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl Voice {
    pub fn new(symbols: Vec<Box<dyn Symbol>>) -> Self {
        Self { symbols }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, symbols) = delimited(char('['), delimited(ws, parse_symbols, ws), char(']'))(input)?;

        Ok((input, Voice::new(symbols)))
    }
}

fn parse_symbols(input: &str) -> IResult<&str, Vec<Box<dyn Symbol>>> {
    let (input, first) = parse(input)?;

    // Ok((input, vec![first]))

    let (input, mut symbols) = many0(preceded(ws, |i| parse_next(i, first.octave(), first.duration())))(input)?;

    symbols.insert(0, first);

    Ok((input, symbols))
}

fn parse(mut input: &str) -> IResult<&str, Box<dyn Symbol>> {
    parse_next(input, 1, Duration::default())
}

fn parse_next(mut input: &str, prev_octave: i8, prev_duration: Duration) -> IResult<&str, Box<dyn Symbol>> {
    let (_, next) = peek(one_of("abcdefghilmrst{_"))(input)?;

    let (input, symbol) = match next {
        '_' => {
            let (input, rest) = Rest::parse_next(input, prev_octave, prev_duration)?;
            let b: Box<dyn Symbol> = Box::new(rest);
            (input, b)
        },
        // "{" => Box::new(Chord::parse(input)?),
        _ => {
            let (input, note) = Note::parse_next(input, prev_octave, prev_duration)?;
            let b: Box<dyn Symbol> = Box::new(note);
            (input, b)
        },
        // _ => return Err(Err::Error(error_position!(input, ErrorKind::NoneOf)));
    };

    // println!("")

    Ok((input, symbol))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::note::Diatonic;
    use crate::rest::Rest;

    use super::*;

    #[test]
    fn parse_one_note() -> Result<()> {
        let (_, voice) = Voice::parse("[ a1 ]")?;

        assert!(voice.symbols[0].equals(&Note::from_name(Diatonic::A)));

        Ok(())
    }

    #[test]
    fn parse_notes_and_rests() -> Result<()> {
        let (_, voice) = Voice::parse("[ a1 _ ]")?;

        assert_eq!(voice.symbols.len(), 2);

        assert!(voice.symbols[0].equals(&Note::from_name(Diatonic::A)));
        assert!(voice.symbols[1].equals(&Rest::default()));

        Ok(())
    }
}
