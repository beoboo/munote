use std::any::Any;

use nom::combinator::opt;
use nom::{bytes::complete::tag, IResult};

use crate::symbol::Symbol;
use crate::{dots::Dots, duration::Duration};

#[derive(Debug, PartialEq)]
pub struct Rest {
    pub octave: i8, // This is a passthrough for using the previous note octave
    pub duration: Duration,
    pub dots: Dots,
}

impl Default for Rest {
    fn default() -> Self {
        Self {
            octave: 1,
            duration: Duration::default(),
            dots: Dots::default(),
        }
    }
}

impl Symbol for Rest {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Symbol) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn octave(&self) -> i8 {
        self.octave
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl Rest {
    pub fn new(octave: i8, duration: Duration, dots: Dots) -> Self {
        Self { octave, duration, dots }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        Self::parse_next(input, 1, Duration::default())
    }

    pub fn parse_next(input: &str, prev_octave: i8, prev_duration: Duration) -> IResult<&str, Self> {
        let (input, _) = tag("_")(input)?;
        let (input, maybe_duration) = opt(Duration::parse)(input)?;
        let (input, dots) = Dots::parse(input)?;

        Ok((
            input,
            Rest::new(prev_octave, maybe_duration.unwrap_or(prev_duration), dots),
        ))
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

    #[test]
    fn same_duration() -> Result<()> {
        let duration = Duration::new(2, 1);
        let (_, rest) = Rest::parse_next("_", 1, duration)?;

        assert_eq!(rest.duration, duration);
        Ok(())
    }
}
