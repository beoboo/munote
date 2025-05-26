use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul};

use nom::{
    bytes::complete::tag,
    character::complete::{char as ch, one_of, u8},
    combinator::{opt, peek},
    IResult,
};

use crate::models::Span;

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

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let n1 = self.num * rhs.denom;
        let n2 = rhs.num * self.denom;

        Self::new(n1 + n2, self.denom * rhs.denom)
    }
}

impl Add<u8> for Duration {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::new(rhs, 1) + self
    }
}

impl Add<Duration> for u8 {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        Duration::new(self, 1) + rhs
    }
}

impl Mul for Duration {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Duration {
    pub fn new(num: u8, denom: u8) -> Self {
        let gcd = gcd(num, denom);

        Self { num: num / gcd, denom: denom / gcd }
    }

    pub fn parse(input: Span) -> IResult<Span, Self> {
        peek(one_of("*/"))(input)?;

        let (input, num) = duration_num(input).unwrap_or((input, 1));
        let (input, denom) = duration_denom(input).unwrap_or((input, 1));
        let (input, _) = opt(tag("ms"))(input)?;

        Ok((input, Self::new(num, denom)))
    }

    pub fn as_f32(&self) -> f32 {
        self.num as f32 / self.denom as f32
    }
}

fn duration_num(input: Span) -> IResult<Span, u8> {
    let (input, _) = ch('*')(input)?;
    u8(input)
}

fn duration_denom(input: Span) -> IResult<Span, u8> {
    let (input, _) = ch('/')(input)?;
    u8(input)
}

fn gcd(mut n: u8, mut m: u8) -> u8 {
    if m == 0 || n == 0 {
        return 1;
    }

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_duration(input: &str) -> Result<Duration> {
        let (input, parsed) =
            Duration::parse(Span::new(input)).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn valid() -> Result<()> {
        assert_duration(parse_duration("*1")?, Duration::new(1, 1), 1.0);
        assert_duration(parse_duration("*2/4")?, Duration::new(2, 4), 0.5);
        assert_duration(parse_duration("/3")?, Duration::new(1, 3), 0.333);
        assert_duration(parse_duration("*5ms")?, Duration::new(5, 1), 5.0);

        Ok(())
    }

    #[test]
    fn add() {
        assert_duration(Duration::new(1, 1) + Duration::new(1, 1), Duration::new(2, 1), 2.0);
        assert_duration(Duration::new(1, 1) + Duration::new(1, 2), Duration::new(3, 2), 1.5);
        assert_duration(Duration::new(1, 2) + Duration::new(1, 2), Duration::new(1, 1), 1.0);
    }

    #[test]
    fn add_scalar() {
        assert_duration(1 + Duration::new(1, 1), Duration::new(2, 1), 2.0);
    }

    fn assert_duration(actual: Duration, e1: Duration, e2: f32) {
        assert_eq!(actual, e1);
        assert!((actual.as_f32() - e2).abs() < 0.001);
    }
}
