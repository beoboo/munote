use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{char, one_of},
    combinator::peek,
    IResult,
    multi::many0,
    sequence::{delimited, preceded, terminated},
};

use crate::{
    comment::all_comments,
    context::ContextPtr,
    models::ws,
    voice::Voice,
};
use crate::models::Span;

#[derive(Debug)]
pub struct Score {
    pub staffs: HashMap<u8, Staff>,
}

#[derive(Debug, Default)]
pub struct Staff {
    pub voices: Vec<Voice>,
}

impl Staff {
    pub fn add_voice(&mut self, voice: Voice) {
        self.voices.push(voice);
    }
}

impl Score {
    pub fn new(voices: Vec<Voice>) -> Self {
        let mut staffs = HashMap::new();

        for voice in voices {
            let id = voice.staff;

            if !staffs.contains_key(&id) {
                staffs.insert(id, Staff::default());
            }

            let staff = staffs.get_mut(&id).unwrap();

            staff.add_voice(voice);
        }

        Self { staffs }
    }

    pub fn parse(input: &str, context: ContextPtr) -> Result<Self> {
        let input = all_comments(input)?;

        let (_, score) = preceded(ws, |s| parse_internal(s, context.clone()))(Span::new(input.as_str()))
            .map_err(|e| anyhow!("{e}"))?;

        Ok(score)
    }
}

fn parse_internal(input: Span, context: ContextPtr) -> IResult<Span, Score> {
    let (_, next) = peek(one_of("[{"))(input)?;

    let (input, voices) = match next {
        '{' => delimited(
            terminated(char('{'), ws),
            |s| parse_voices(s, context.clone()),
            terminated(char('}'), ws),
        )(input)?,
        _ => {
            let (input, voice) = Voice::parse(input, context)?;
            (input, vec![voice])
        }
    };

    Ok((input, Score::new(voices)))
}

fn parse_voices(input: Span, context: ContextPtr) -> IResult<Span, Vec<Voice>> {
    let (input, first) = Voice::parse(input, context.clone())?;

    let (input, mut voices) =
        many0(preceded(terminated(char(','), ws), |i| {
            Voice::parse(i, context.clone())
        }))(input)?;

    voices.insert(0, first);

    Ok((input, voices))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_score(input: &str) -> Result<Score> {
        let context = ContextPtr::default();

        let score =
            Score::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        Ok(score)
    }

    #[test]
    fn parse_one_voice() -> Result<()> {
        let score = parse_score("[ a1 ]")?;
        assert_eq!(score.staffs.len(), 1);

        let staff = &score.staffs[&1];
        assert_eq!(staff.voices.len(), 1);

        Ok(())
    }

    #[test]
    fn parse_multiple_voices() -> Result<()> {
        let score = parse_score(
            "\
{
  [ a1 ],
  [ b1 ]
}",
        )?;

        assert_eq!(score.staffs.len(), 1);

        let staff = &score.staffs[&1];

        assert_eq!(staff.voices.len(), 2);

        Ok(())
    }

    #[test]
    fn skip_comments() -> Result<()> {
        let score = parse_score(
            "\
(* this is a comment *)
{
  [ a1 ],% and this too
  [ b1 ]
}",
        )?;

        assert_eq!(score.staffs.len(), 1);

        Ok(())
    }

    #[test]
    fn invalid_score() {
        let res = parse_score("{ [ \\unknown ] }");
        println!("{res:?}");

        assert!(parse_score("{ [ \\accelerando ] }").is_err());
    }
}
