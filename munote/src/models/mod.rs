use nom::{
    bytes::complete::take_while,
    character::complete::{alpha1, alphanumeric0},
    combinator::recognize,
    IResult,
    sequence::preceded,
};
use nom_locate::LocatedSpan;

pub mod accidentals;
pub mod chord;
pub mod comment;
pub mod context;
pub mod dots;
pub mod duration;
pub mod note;
pub mod rest;
pub mod score;
pub mod event;
pub mod tag;
pub mod tag_id;
pub mod tag_param;
pub mod unit;
pub mod voice;
pub mod error;
pub mod tag_validator;
pub mod tag_definitions;
pub mod symbols;
pub mod display_event;

type Span<'a> = LocatedSpan<&'a str>;

fn string(input: Span) -> IResult<Span, Span> {
    recognize(preceded(alpha1, alphanumeric0))(input)
}

fn ws(input: Span) -> IResult<Span, Span> {
    take_while(is_whitespace)(input)
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn whitespaces() -> Result<()> {
        let (input, res) = ws(Span::new(" \t\n"))?;

        assert_eq!(*input.fragment(), "");
        assert_eq!(res, Span::new(" \t\n"));

        Ok(())
    }
}
