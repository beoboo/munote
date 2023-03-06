use std::any::Any;

use nom::{bytes::complete::tag, combinator::opt, IResult};

use crate::{
    context::ContextPtr,
    dots::Dots,
    duration::Duration,
    impl_symbol_for,
    models::ws,
    event::Event,
};
use crate::models::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Rest {
    pub duration: Duration,
    pub dots: Dots,
}

impl_symbol_for!(Rest);

impl Default for Rest {
    fn default() -> Self {
        Self {
            duration: Duration::default(),
            dots: Dots::default(),
        }
    }
}

impl Rest {
    pub fn new(duration: Duration, dots: Dots) -> Self {
        Self { duration, dots }
    }

    pub fn parse(input: Span, context: ContextPtr) -> IResult<Span, Self> {
        let (input, _) = tag("_")(input)?;
        let (input, maybe_duration) = opt(Duration::parse)(input)?;
        let (input, dots) = Dots::parse(input)?;

        // Eat remaining whitespaces
        let (input, _) = ws(input)?;

        Ok((
            input,
            Rest::new(
                maybe_duration.unwrap_or(context.borrow().duration),
                dots,
            ),
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

        let (input, parsed) =
            Rest::parse(Span::new(input), context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
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

        let (_, rest) = Rest::parse(Span::new("_"), context)?;

        assert_eq!(rest.duration, duration);
        Ok(())
    }
}
