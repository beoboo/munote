use std::{convert::From, str::FromStr};
use std::any::Any;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i8, one_of},
    combinator::{map, map_res, opt, value},
    IResult, Parser,
};
use parse_display::FromStr;

use crate::{accidentals::Accidentals, dots::Dots, duration::Duration};
use crate::symbol::Symbol;

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

#[derive(Debug, PartialEq)]
pub struct Note {
    pub name: NoteName,
    octave: i8,
    pub accidentals: Accidentals,
    duration: Duration,
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

    pub fn from_name(name: impl Into<NoteName>) -> Self {
        Self::new(name, Accidentals::Natural, 1, Duration::default(), Dots::None)
    }

    pub fn with_duration(mut self, num: u8, denom: u8) -> Self {
        self.duration = Duration::new(num, denom);
        self
    }

    pub fn with_octave(mut self, octave: i8) -> Self {
        self.octave = octave;
        self
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        Self::parse_next(input, 1, Duration::default())
    }

    pub fn parse_next(input: &str, prev_octave: i8, prev_duration: Duration) -> IResult<&str, Self> {
        let (input, name) = alt((
            map(Chromatic::parse, |c| NoteName::from(c)),
            map(Diatonic::parse, |d| NoteName::from(d)),
            map(Solfege::parse, |s| NoteName::from(s)),
        ))
        .parse(input)?;

        let (input, accidentals) = Accidentals::parse(input)?;
        let (input, maybe_octave) = opt(i8).parse(input)?;
        let (input, maybe_duration) = opt(Duration::parse)(input)?;
        let (input, dots) = Dots::parse(input)?;

        println!("Duration: {maybe_duration:?}");
        Ok((
            input,
            Note::new(
                name,
                accidentals,
                maybe_octave.unwrap_or(prev_octave),
                maybe_duration.unwrap_or(prev_duration),
                dots,
            ),
        ))
    }
}

impl Symbol for Note {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Symbol) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn octave(&self) -> i8 {
        self.octave
    }

    fn duration(&self) -> Duration {
        self.duration
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

    #[test]
    fn same_octave() -> Result<()> {
        let (_, note) = Note::parse_next("c", 2, Duration::default())?;
        assert_note(&note, Diatonic::C, 2);

        Ok(())
    }

    #[test]
    fn same_duration() -> Result<()> {
        let duration = Duration::new(2, 1);
        let (_, note) = Note::parse_next("c", 1, duration)?;

        assert_eq!(note.duration, duration);
        Ok(())
    }

    fn assert_note(note: &Note, name: impl Into<NoteName>, octave: i8) {
        assert_eq!(note.name, name.into());
        assert_eq!(note.octave, octave);
    }
}
