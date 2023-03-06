use nom::{
    bytes::complete::tag,
    character::complete::{char as ch, u8},
    combinator::opt,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub num: u8,
    pub denom: u8,
}

impl Default for Duration {
    fn default() -> Self {
        Self { num: 1, denom: 1 }
    }
}

impl Duration {
    pub fn new(num: u8, denom: u8) -> Self {
        Self { num, denom }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, num) = duration_num(input).unwrap_or((input, 1));
        let (input, denom) = duration_denom(input).unwrap_or((input, 1));
        let (input, _) = opt(tag("ms"))(input)?;

        Ok((input, Self { num, denom }))
    }
}

fn duration_num(input: &str) -> IResult<&str, u8> {
    let (input, _) = ch('*')(input)?;
    u8(input)
}

fn duration_denom(input: &str) -> IResult<&str, u8> {
    let (input, _) = ch('/')(input)?;
    u8(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parse_duration() -> Result<()> {
        let (_, duration) = Duration::parse("*1")?;
        assert_eq!(duration, Duration::new(1, 1));

        let (_, duration) = Duration::parse("*2/4")?;
        assert_eq!(duration, Duration::new(2, 4));

        let (_, duration) = Duration::parse("/3")?;
        assert_eq!(duration, Duration::new(1, 3));

        let (_, duration) = Duration::parse("*5ms")?;
        assert_eq!(duration, Duration::new(5, 1));

        Ok(())
    }
}
