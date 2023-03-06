use nom::{bytes::complete::take_while_m_n, IResult, Parser};
use crate::models::Span;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Dots {
    #[default]
    None,
    Single,
    Double,
    Triple,
}

impl Dots {
    pub fn parse(input: Span) -> IResult<Span, Self> {
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
    use anyhow::{anyhow, Result};

    fn parse_dots(input: &str) -> Result<Dots> {
        let (input, parsed) =
            Dots::parse(Span::new(input)).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn parse_valid() -> Result<()> {
        assert_eq!(parse_dots("")?, Dots::None);
        assert_eq!(parse_dots(".")?, Dots::Single);
        assert_eq!(parse_dots("..")?, Dots::Double);
        assert_eq!(parse_dots("...")?, Dots::Triple);

        Ok(())
    }
}
