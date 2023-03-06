use std::{any::Any, fmt};

use nom::{
    character::complete::{char, one_of},
    combinator::{opt, peek},
    IResult,
    sequence::{preceded, terminated},
};
use nom::multi::fold_many0;

use crate::{
    chord::Chord,
    context::ContextPtr,
    models::ws,
    note::Note,
    rest::Rest,
    tag::Tag,
};

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
//
// impl PartialEq<Box<dyn Event>> for Box<dyn Event> {
//     fn eq(&self, other: &(Box<dyn Event>)) -> bool {
//         self.equals(other)
//     }
// }
//
// impl PartialEq<Box<dyn Event>> for Box<dyn Event> {
//     fn eq(&self, other: &Box<dyn Event>) -> bool {
//         self.as_ref() == other.as_ref()
//     }
// }

pub fn same_symbols(
    lhs: &Vec<Box<dyn Event>>,
    rhs: &Vec<Box<dyn Event>>,
) -> bool {
    lhs.len() == rhs.len()
        && lhs
        .iter()
        .enumerate()
        .fold(true, |val, (i, s)| val && s.equals(&*rhs[i]))
}

pub fn parse_delimited_events(
    mut input: &str,
    context: ContextPtr,
    start_delimiter: char,
    end_delimiter: char,
    skip_start_delimiter: bool,
)
    -> IResult<&str, Vec<Box<dyn Event>>> {
    // println!("Parsing symbols: {input}");
    if !skip_start_delimiter {
        let (i, _) = terminated(char(start_delimiter), ws)(input)?;
        input = i;
    }

    let (input, (skip_end_delimiter, symbols)) = parse_symbols(input, context.clone())?;

    if !skip_end_delimiter {
        // println!("Not already terminated!");
        let (input, _) = terminated(char(end_delimiter), ws)(input)?;
        return Ok((input, symbols));
    }

    // println!("Already terminated!");
    Ok((input, symbols))
}

fn parse_symbols(
    input: &str,
    context: ContextPtr,
) -> IResult<&str, (bool, Vec<Box<dyn Event>>)> {
    // println!("Checking symbols: \"{input}\"");
    let (input, first) = parse_symbol(input, context.clone())?;

    let first = first.expect("No initial symbol found");

    let (input, (already_terminated, mut symbols)) = fold_many0(
        preceded(
            terminated(opt(char(',')), ws),
            preceded(ws, |i| parse_symbol(i, context.clone())),
        ),
        || (false, Vec::new()),
        |(mut already_terminated, mut symbols): (bool, Vec<Box<dyn Event>>), maybe_symbol| {
            match maybe_symbol {
                Some(s) => symbols.push(s),
                None => already_terminated = true,
            }

            (already_terminated, symbols)
        },
    )(input)?;

    symbols.insert(0, first);
    // println!("Parsed symbols: \"{symbols:?}\"");

    Ok((input, (already_terminated, symbols)))
}

fn parse_symbol(
    input: &str,
    context: ContextPtr,
) -> IResult<&str, Option<Box<dyn Event>>> {
    // println!("Checking symbol: \"{input}\"");
    let (_, next) = peek(one_of("abcdefghilmrst{_|\\"))(input)?;

    let (input, symbol) = match next {
        '\\' | '|' => {
            let (input, maybe_tag) = Tag::parse(input, context)?;

            let tag = match maybe_tag {
                Some(tag) => tag,
                None => return Ok((input, None))
            };

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

    Ok((input, Some(symbol)))
}
