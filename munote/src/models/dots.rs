use std::{convert::From, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while_m_n},
    character::complete::{char as ch, i8, one_of, u8},
    combinator::{map, map_res, opt, value},
    IResult,
    Parser,
};
use parse_display::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Dots {
    None,
    Single,
    Double,
    Triple,
}

impl Dots {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, dots) = take_while_m_n(0, 3, |c| c == '.').parse(input)?;

        let res = match dots.len() {
            0 => Dots::None,
            1 => Dots::Single,
            2 => Dots::Double,
            _ => Dots::Triple,
        };

        Ok((input, res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_dots() -> Result<()> {
        let (_, dots) = Dots::parse("")?;
        assert_eq!(dots, Dots::None);

        let (_, dots) = Dots::parse(".")?;
        assert_eq!(dots, Dots::Single);

        let (_, dots) = Dots::parse("..")?;
        assert_eq!(dots, Dots::Double);

        let (_, dots) = Dots::parse("...")?;
        assert_eq!(dots, Dots::Triple);

        Ok(())
    }
}
