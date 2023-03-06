use std::{convert::From, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i8, one_of},
    combinator::{map, map_res, opt, value},
    IResult,
    Parser,
};
use parse_display::FromStr;

use crate::{accidentals::Accidentals, dots::Dots, duration::Duration};

#[derive(Debug, Clone, FromStr, PartialEq)]
#[display(style = "lowercase")]
pub enum Diatonic {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Diatonic {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map_res(one_of("abcdefgh"), |d| Self::from_str(&d.to_string()))(input)
    }
}

impl From<Diatonic> for NoteName {
    fn from(d: Diatonic) -> Self {
        Self::Diatonic(d)
    }
}

#[derive(Debug, Clone, FromStr, PartialEq)]
#[display(style = "lowercase")]
pub enum Chromatic {
    Cis,
    Dis,
    Fis,
    Gis,
    Ais,
}

impl Chromatic {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Chromatic::Cis, tag("cis")),
            value(Chromatic::Dis, tag("dis")),
            value(Chromatic::Fis, tag("fis")),
            value(Chromatic::Gis, tag("gis")),
            value(Chromatic::Ais, tag("ais")),
        ))(input)
    }
}

impl From<Chromatic> for NoteName {
    fn from(c: Chromatic) -> Self {
        Self::Chromatic(c)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Solfege {
    Do,
    Re,
    Me,
    Fa,
    Sol,
    La,
    Si,
    Ti,
}

impl Solfege {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Solfege::Do, tag("do")),
            value(Solfege::Re, tag("re")),
            value(Solfege::Me, tag("me")),
            value(Solfege::Fa, tag("fa")),
            value(Solfege::Sol, tag("sol")),
            value(Solfege::La, tag("la")),
            value(Solfege::Si, tag("si")),
            value(Solfege::Ti, tag("ti")),
        ))(input)
    }
}

impl From<Solfege> for NoteName {
    fn from(s: Solfege) -> Self {
        Self::Solfege(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum NoteName {
    Diatonic(Diatonic),
    Chromatic(Chromatic),
    Solfege(Solfege),
}

pub struct Note {
    pub name: NoteName,
    pub octave: i8,
    pub accidentals: Accidentals,
    pub duration: Duration,
    pub dots: Dots,
}

impl Note {
    pub fn new(
        name: impl Into<NoteName>,
        accidentals: Accidentals,
        octave: i8,
        duration: Duration,
        dots: Dots,
    ) -> Self {
        Self {
            name: name.into(),
            accidentals,
            octave,
            duration,
            dots,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = alt((
            map(Chromatic::parse, |c| NoteName::from(c)),
            map(Diatonic::parse, |d| NoteName::from(d)),
            map(Solfege::parse, |s| NoteName::from(s)),
        ))
        .parse(input)?;

        let (input, accidentals) = Accidentals::parse(input)?;
        let (input, maybe_octave) = opt(i8).parse(input)?;
        let (input, duration) = Duration::parse(input)?;
        let (input, dots) = Dots::parse(input)?;

        Ok((
            input,
            Note::new(name, accidentals, maybe_octave.unwrap_or(1), duration, dots),
        ))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::accidentals::Accidentals;

    use super::*;

    #[test]
    fn parse_diatonic() -> Result<()> {
        let (_, note) = Note::parse("a1")?;
        assert_note(&note, Diatonic::A, 1);

        let (_, note) = Note::parse("f")?;
        assert_note(&note, Diatonic::F, 1);

        Ok(())
    }

    #[test]
    fn parse_chromatic() -> Result<()> {
        let (_, note) = Note::parse("cis2")?;
        assert_note(&note, Chromatic::Cis, 2);

        Ok(())
    }

    #[test]
    fn parse_solfege() -> Result<()> {
        let (_, note) = Note::parse("sol-1")?;
        assert_note(&note, Solfege::Sol, -1);

        Ok(())
    }

    #[test]
    fn parse_accidentals() -> Result<()> {
        let (_, note) = Note::parse("c#1")?;
        assert_eq!(note.accidentals, Accidentals::Sharp);

        Ok(())
    }

    #[test]
    fn parse_duration() -> Result<()> {
        let (_, note) = Note::parse("c*1")?;
        assert_eq!(note.duration, Duration::new(1, 1));

        Ok(())
    }

    #[test]
    fn parse_dots() -> Result<()> {
        let (_, note) = Note::parse("c.")?;
        assert_eq!(note.dots, Dots::Single);

        Ok(())
    }

    fn assert_note(note: &Note, name: impl Into<NoteName>, octave: i8) {
        assert_eq!(note.name, name.into());
        assert_eq!(note.octave, octave);
    }
}
