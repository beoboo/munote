use std::cmp::Ordering;
use nom::{
    bytes::complete::tag,
    character::complete::{char as ch, u8},
    combinator::opt,
    IResult,
};
use nom::character::complete::one_of;
use nom::combinator::{peek, verify};
use nom::sequence::preceded;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    pub num: u8,
    pub denom: u8,
}

impl Default for Duration {
    fn default() -> Self {
        Self { num: 1, denom: 1 }
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_f32().partial_cmp(&other.as_f32())
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_f32().total_cmp(&other.as_f32())
    }
}

impl Duration {
    pub fn new(num: u8, denom: u8) -> Self {
        Self { num, denom }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        peek(one_of("*/"))(input)?;

        let (input, num) = duration_num(input).unwrap_or((input, 1));
        let (input, denom) = duration_denom(input).unwrap_or((input, 1));
        let (input, _) = opt(tag("ms"))(input)?;

        Ok((input, Self { num, denom }))
    }

    pub fn as_f32(&self) -> f32 {
        self.num as f32 / self.denom as f32
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
