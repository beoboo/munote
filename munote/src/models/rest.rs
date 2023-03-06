use nom::{bytes::complete::tag, IResult};

use crate::{dots::Dots, duration::Duration};

pub struct Rest {
    pub duration: Duration,
    pub dots: Dots,
}

impl Rest {
    pub fn new(duration: Duration, dots: Dots) -> Self {
        Self { duration, dots }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("_")(input)?;
        let (input, duration) = Duration::parse(input)?;
        let (input, dots) = Dots::parse(input)?;

        Ok((input, Rest::new(duration, dots)))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let (_, rest) = Rest::parse("_")?;
        assert_eq!(rest.duration, Duration::new(1, 1));
        assert_eq!(rest.dots, Dots::None);

        Ok(())
    }

    #[test]
    fn parse_duration() -> Result<()> {
        let (_, rest) = Rest::parse("_*5/2")?;
        assert_eq!(rest.duration, Duration::new(5, 2));

        Ok(())
    }

    #[test]
    fn parse_dots() -> Result<()> {
        let (_, rest) = Rest::parse("_.")?;
        assert_eq!(rest.dots, Dots::Single);

        Ok(())
    }
}
