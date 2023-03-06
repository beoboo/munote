use crate::duration::Duration;
use std::{any::Any, fmt::Debug};
use nom::character::complete::one_of;
use nom::combinator::peek;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::preceded;
use crate::chord::Chord;
use crate::context::ContextPtr;
use crate::models::ws;
use crate::note::Note;
use crate::rest::Rest;
use crate::tag::Tag;

pub trait Symbol: Debug {
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, _: &dyn Symbol) -> bool;

    fn clone_box(&self) -> Box<dyn Symbol>;

    fn octave(&self) -> i8;

    fn duration(&self) -> Duration;
}

impl Clone for Box<dyn Symbol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn parse_symbols(input: &str, context: ContextPtr) -> IResult<&str, Vec<Box<dyn Symbol>>> {
    let (input, first) = parse_symbol(input, context.clone())?;

    let (input, mut symbols) = many0(preceded(ws, |i| parse_symbol(i, context.clone())))(input)?;

    symbols.insert(0, first);

    Ok((input, symbols))
}

fn parse_symbol(input: &str, context: ContextPtr) -> IResult<&str, Box<dyn Symbol>> {
    let (_, next) = peek(one_of("abcdefghilmrst{_\\"))(input)?;

    let (input, symbol) = match next {
        '\\' => {
            let (input, tag) = Tag::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(tag);
            (input, b)
        },
        '{' => {
            let (input, chord) = Chord::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(chord);
            (input, b)
        },
        '_' => {
            let (input, rest) = Rest::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(rest);
            (input, b)
        },
        _ => {
            let (input, note) = Note::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(note);
            (input, b)
        },
    };

    Ok((input, symbol))
}

