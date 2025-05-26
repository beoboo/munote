use nom::{bytes::complete::take_while_m_n, IResult, Parser};

use crate::duration::Duration;
use crate::models::Span;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Dots {
    #[default]
    None,
    Single,
    Double,
    Triple,
}

impl From<Dots> for usize {
    fn from(dots: Dots) -> Self {
        match dots {
            Dots::None => 0,
            Dots::Single => 1,
            Dots::Double => 2,
            Dots::Triple => 3,
        }
    }
}

impl Dots {
    pub fn duration(&self) -> Duration {
        match self {
            Self::None => Duration::new(0, 1),
            Self::Single => Duration::new(1, 2),
            Self::Double => Duration::new(3, 4),
            Self::Triple => Duration::new(7, 8),
        }
    }
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
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_dots(input: &str) -> Result<Dots> {
        let (input, parsed) =
            Dots::parse(Span::new(input)).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn parse_valid() -> Result<()> {
        assert_dots(parse_dots("")?, Dots::None, 0, Duration::new(0, 1));
        assert_dots(parse_dots(".")?, Dots::Single, 1, Duration::new(1, 2));
        assert_dots(parse_dots("..")?, Dots::Double, 2, Duration::new(3, 4));
        assert_dots(parse_dots("...")?, Dots::Triple, 3, Duration::new(7, 8));

        Ok(())
    }

    fn assert_dots(dots: Dots, expected: Dots, len: usize, duration: Duration) {
        assert_eq!(dots, expected);
        assert_eq!(dots as usize, len);
        assert_eq!(dots.duration(), duration);
    }
}
