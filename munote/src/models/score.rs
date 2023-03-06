use anyhow::{anyhow, Result};
use nom::{
    character::complete::{char, one_of},
    combinator::peek,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::models::comma;
use crate::voice::Voice;
use crate::{context::ContextPtr, models::ws};

#[derive(Debug)]
pub struct Score {
    pub voices: Vec<Voice>,
}

impl Score {
    pub fn new(voices: Vec<Voice>) -> Self {
        Self { voices }
    }

    pub fn parse(input: &str, context: ContextPtr) -> Result<Self> {
        let (_, score) = Self::parse_internal(input, context)
            .map_err(|e| anyhow!("{e}"))?;

        Ok(score)
    }

    fn parse_internal(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (_, next) = peek(one_of("[{"))(input)?;

        let (input, voices) = match next {
            '{' => delimited(
                char('{'),
                delimited(ws, |s| parse_voices(s, context.clone()), ws),
                char('}'),
            )(&input)?,
            _ => {
                let (input, voice) = Voice::parse(&input, context)?;
                (input, vec![voice])
            },
        };

        Ok((input, Self::new(voices)))
    }
}

fn parse_voices(input: &str, context: ContextPtr) -> IResult<&str, Vec<Voice>> {
    let (input, first) = Voice::parse(input, context.clone())?;

    let (input, mut voices) = many0(preceded(comma, |i| Voice::parse(i, context.clone())))(input)?;

    voices.insert(0, first);

    Ok((input, voices))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_score(input: &str) -> Result<Score> {
        let context = ContextPtr::default();

        let score = Score::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        Ok(score)
    }

    #[test]
    fn parse_one_voice() -> Result<()> {
        let score = parse_score("[ a1 ]")?;

        assert_eq!(score.voices.len(), 1);

        Ok(())
    }

    #[test]
    fn parse_multiple_voice() -> Result<()> {
        let score = parse_score("{ [ a1 ], [ b1 ] }")?;

        assert_eq!(score.voices.len(), 2);

        Ok(())
    }
}
