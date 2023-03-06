use nom::{bytes::complete::take_while_m_n, IResult, Parser};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Dots {
    #[default]
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
