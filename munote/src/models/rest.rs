use std::any::Any;

use nom::{bytes::complete::tag, combinator::opt, IResult};

use crate::{context::ContextPtr, dots::Dots, duration::Duration, symbol::Symbol};

#[derive(Clone, Debug, PartialEq)]
pub struct Rest {
    pub duration: Duration,
    pub dots: Dots,
}

impl Default for Rest {
    fn default() -> Self {
        Self {
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

    fn clone_box(&self) -> Box<dyn Symbol> {
        Box::new((*self).clone())
    }

    fn octave(&self) -> i8 {
        1
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl Rest {
    pub fn new(duration: Duration, dots: Dots) -> Self {
        Self { duration, dots }
    }

    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, _) = tag("_")(input)?;
        let (input, maybe_duration) = opt(Duration::parse)(input)?;
        let (input, dots) = Dots::parse(input)?;

        Ok((
            input,
            Rest::new(maybe_duration.unwrap_or(context.borrow().duration), dots),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{Context, Ptr};
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_rest(input: &str) -> Result<Rest> {
        let context = ContextPtr::default();

        let (input, rest) = Rest::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert!(input.is_empty());

        Ok(rest)
    }

    #[test]
    fn parse() -> Result<()> {
        let rest = parse_rest("_")?;
        assert_eq!(rest.duration, Duration::new(1, 1));
        assert_eq!(rest.dots, Dots::None);

        Ok(())
    }

    #[test]
    fn parse_duration() -> Result<()> {
        let rest = parse_rest("_*5/2")?;
        assert_eq!(rest.duration, Duration::new(5, 2));

        Ok(())
    }

    #[test]
    fn parse_dots() -> Result<()> {
        let rest = parse_rest("_.")?;
        assert_eq!(rest.dots, Dots::Single);

        Ok(())
    }

    #[test]
    fn same_duration() -> Result<()> {
        let duration = Duration::new(2, 1);
        let context = Ptr::new(Context {
            duration,
            ..Default::default()
        });

        let (_, rest) = Rest::parse("_", context)?;

        assert_eq!(rest.duration, duration);
        Ok(())
    }
}
