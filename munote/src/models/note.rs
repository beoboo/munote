use std::{any::Any, convert::From, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i8, one_of},
    combinator::{map, map_res, opt, value},
    IResult,
    Parser,
};
use parse_display::FromStr;

use crate::{
    accidentals::Accidentals,
    context::ContextPtr,
    dots::Dots,
    duration::Duration,
    impl_symbol_for,
    models::ws,
    event::Event,
};
use crate::models::Span;

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub name: NoteName,
    pub octave: i8,
    pub accidentals: Accidentals,
    pub duration: Duration,
    pub dots: Dots,
}

impl_symbol_for!(Note);

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
        Self::new(
            name,
            Accidentals::Natural,
            1,
            Duration::default(),
            Dots::None,
        )
    }

    pub fn with_duration(mut self, num: u8, denom: u8) -> Self {
        self.duration = Duration::new(num, denom);
        self
    }

    pub fn with_octave(mut self, octave: i8) -> Self {
        self.octave = octave;
        self
    }

    pub fn with_accidentals(mut self, accidentals: Accidentals) -> Self {
        self.accidentals = accidentals;
        self
    }

    pub fn parse(input: Span, mut context: ContextPtr) -> IResult<Span, Self> {
        let (input, name) = alt((
            map(Chromatic::parse, |c| NoteName::from(c)),
            map(Diatonic::parse, |d| NoteName::from(d)),
            map(Solfege::parse, |s| NoteName::from(s)),
        ))(input)?;

        let (input, accidentals) = Accidentals::parse(input)?;
        let (input, maybe_octave) = opt(i8)(input)?;
        let (input, maybe_duration) = opt(Duration::parse)(input)?;
        let (input, dots) = Dots::parse(input)?;

        // Eat remaining whitespaces
        let (input, _) = ws(input)?;

        let mut context = context.borrow_mut();
        let octave = maybe_octave.unwrap_or(context.octave);
        let duration = maybe_duration.unwrap_or(context.duration);

        context.octave = octave;
        context.duration = duration;

        Ok((input, Note::new(name, accidentals, octave, duration, dots)))
    }
}

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
    pub fn parse(input: Span) -> IResult<Span, Self> {
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
    pub fn parse(input: Span) -> IResult<Span, Self> {
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
    pub fn parse(input: Span) -> IResult<Span, Self> {
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

#[derive(Clone, Debug, PartialEq)]
pub enum NoteName {
    Diatonic(Diatonic),
    Chromatic(Chromatic),
    Solfege(Solfege),
}
#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::{
        accidentals::Accidentals,
        context::{Context, Ptr},
    };

    use super::*;

    fn parse_note(input: &str) -> Result<Note> {
        let context = ContextPtr::default();

        let (input, parsed) =
            Note::parse(Span::new(input), context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn parse_diatonic() -> Result<()> {
        let note = parse_note("a1")?;
        assert_note(&note, Diatonic::A, 1);

        let note = parse_note("f")?;
        assert_note(&note, Diatonic::F, 1);

        Ok(())
    }

    #[test]
    fn parse_chromatic() -> Result<()> {
        let note = parse_note("cis2")?;
        assert_note(&note, Chromatic::Cis, 2);

        Ok(())
    }

    #[test]
    fn parse_solfege() -> Result<()> {
        let note = parse_note("sol-1")?;
        assert_note(&note, Solfege::Sol, -1);

        Ok(())
    }

    #[test]
    fn parse_accidentals() -> Result<()> {
        let note = parse_note("c#1")?;
        assert_eq!(note.accidentals, Accidentals::Sharp);

        Ok(())
    }

    #[test]
    fn parse_duration() -> Result<()> {
        let note = parse_note("c*1")?;
        assert_eq!(note.duration, Duration::new(1, 1));

        Ok(())
    }

    #[test]
    fn parse_dots() -> Result<()> {
        let note = parse_note("c.")?;
        assert_eq!(note.dots, Dots::Single);

        Ok(())
    }

    #[test]
    fn same_octave() -> Result<()> {
        let context = Ptr::new(Context {
            octave: 2,
            ..Default::default()
        });

        let (_, note) = Note::parse("c".into(), context)?;
        assert_note(&note, Diatonic::C, 2);

        Ok(())
    }

    #[test]
    fn same_duration() -> Result<()> {
        let duration = Duration::new(2, 1);
        let context = Ptr::new(Context {
            duration,
            ..Default::default()
        });

        let (_, note) = Note::parse(Span::new("c"), context)?;

        assert_eq!(note.duration, duration);
        Ok(())
    }

    fn assert_note(note: &Note, name: impl Into<NoteName>, octave: i8) {
        assert_eq!(note.name, name.into());
        assert_eq!(note.octave, octave);
    }
}
