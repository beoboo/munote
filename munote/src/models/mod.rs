use nom::{
    bytes::complete::take_while,
    character::complete::{alpha1, alphanumeric0},
    combinator::recognize,
    sequence::preceded,
    IResult,
};

pub mod accidentals;
pub mod chord;
pub mod comment;
pub mod context;
pub mod dots;
pub mod duration;
pub mod note;
pub mod rest;
pub mod score;
pub mod symbol;
pub mod tag;
pub mod tag_id;
pub mod tag_param;
pub mod unit;
pub mod voice;

fn string(input: &str) -> IResult<&str, &str> {
    recognize(preceded(alpha1, alphanumeric0))(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    take_while(is_whitespace)(input)
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn whitespaces() -> Result<()> {
        let (input, res) = ws(" \t")?;

        assert_eq!(input, "");
        assert_eq!(res, " \t");

        Ok(())
    }
}
