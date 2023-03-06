use std::{any::Any, fmt};

use nom::{
    character::complete::{char, one_of},
    combinator::{opt, peek},
    IResult,
    sequence::{preceded, terminated},
};
use nom::multi::many0;
use nom::sequence::delimited;

use crate::{
    chord::Chord,
    context::ContextPtr,
    models::ws,
    note::Note,
    rest::Rest,
    tag::Tag,
};
use crate::models::Span;

pub trait Event {
    fn as_any(&self) -> &dyn Any;

    fn type_name(&self) -> &'static str;

    fn as_debug(&self) -> &dyn fmt::Debug;

    fn equals(&self, _: &dyn Event) -> bool;

    fn clone_box(&self) -> Box<dyn Event>;
}

#[macro_export]
macro_rules! impl_symbol_for {
    ($t:ty) => {
        impl Event for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn type_name(&self) -> &'static str {
                stringify!($t)
            }

            fn as_debug(&self) -> &dyn std::fmt::Debug {
                self
            }

            fn equals(&self, other: &dyn Event) -> bool {
                other
                    .as_any()
                    .downcast_ref::<Self>()
                    .map_or(false, |a| self == a)
            }

            fn clone_box(&self) -> Box<dyn Event> {
                Box::new((*self).clone())
            }
        }
    };
}

impl Clone for Box<dyn Event> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for dyn Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event({}) {:?}", self.type_name(), self.as_debug())
    }
}

impl<'a, 'b> PartialEq<dyn Event + 'b> for dyn Event + 'a {
    fn eq(&self, other: &(dyn Event + 'b)) -> bool {
        self.equals(other)
    }
}

pub fn parse_delimited_events(
    input: Span,
    context: ContextPtr,
    start_delimiter: char,
    end_delimiter: char,
)
    -> IResult<Span, Vec<Box<dyn Event>>> {

    let (input, events) = delimited(
        terminated(char(start_delimiter), ws),
        |i| parse_events(i, context.clone()),
        terminated(char(end_delimiter), ws))(input)?;

    // println!("Parsed delimited events: \"{events:?}\"");
    // println!("Remaining: \"{input}\"");

    Ok((input, events))
}

fn parse_events(
    input: Span,
    context: ContextPtr,
) -> IResult<Span, Vec<Box<dyn Event>>> {
    // println!("Checking symbols: \"{input}\"");
    let (input, first) = parse_event(input, context.clone())?;

    let (input, mut events) = many0(preceded(
        terminated(opt(char(',')), ws),
        preceded(ws, |i| parse_event(i, context.clone())),
    ))(input)?;

    events.insert(0, first);
    // println!("Parsed events: \"{events:?}\"");
    // println!("Remaining: \"{input}\"");

    Ok((input, events))
}

fn parse_event(
    input: Span,
    context: ContextPtr,
) -> IResult<Span, Box<dyn Event>> {
    // println!("Checking symbol: \"{input}\"");
    let (_, next) = peek(one_of("abcdefghilmrst{_|\\"))(input)?;

    let (input, symbol) = match next {
        '\\' | '|' => {
            let (input, tag) = Tag::parse(input, context)?;
            let b: Box<dyn Event> = Box::new(tag);
            (input, b)
        }
        '{' => {
            let (input, chord) = Chord::parse(input, context)?;
            let b: Box<dyn Event> = Box::new(chord);
            (input, b)
        }
        '_' => {
            let (input, rest) = Rest::parse(input, context)?;
            let b: Box<dyn Event> = Box::new(rest);
            (input, b)
        }
        _ => {
            let (input, note) = Note::parse(input, context)?;
            let b: Box<dyn Event> = Box::new(note);
            (input, b)
        }
    };
    // println!("Parsed symbol: \"{symbol:?}\"");

    // Skip remaining whitespaces
    let (input, _) = ws(input)?;

    Ok((input, symbol))
}
