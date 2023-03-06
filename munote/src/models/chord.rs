use std::any::Any;
use std::convert::From;
use std::str::FromStr;

use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

use crate::context::ContextPtr;
use crate::duration::Duration;
use crate::models::{comma, ws};
use crate::note::Note;
use crate::symbol::Symbol;

#[derive(Debug, PartialEq)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: Duration,
}

impl Symbol for Chord {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Symbol) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn octave(&self) -> i8 {
        1
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl Chord {
    pub fn new(notes: Vec<Note>) -> Self {
        let duration = notes.iter().map(|n| n.duration()).max().unwrap();

        Self { notes, duration }
    }

    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, notes) = delimited(
            char('{'),
            delimited(ws, |i| parse_notes(i, context.clone()), ws),
            char('}'),
        )(input)?;

        Ok((input, Chord::new(notes)))
    }
}

fn parse_notes(input: &str, context: ContextPtr) -> IResult<&str, Vec<Note>> {
    let (input, first) = Note::parse(input, context.clone())?;

    let (input, mut notes) = many0(preceded(comma, |i| Note::parse(i, context.clone())))(input)?;

    notes.insert(0, first);

    Ok((input, notes))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::note::Diatonic;

    use super::*;

    fn parse_chord(input: &str) -> Result<Chord> {
        let context = ContextPtr::default();

        let (_, chord) = Chord::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        Ok(chord)
    }

    #[test]
    fn parse_one() -> Result<()> {
        let chord = parse_chord("{ a1 }")?;

        assert_eq!(chord.notes, vec![Note::from_name(Diatonic::A)]);

        Ok(())
    }

    #[test]
    fn parse_multiple() -> Result<()> {
        let chord = parse_chord("{ a1, b1 }")?;

        assert_eq!(
            chord.notes,
            vec![Note::from_name(Diatonic::A), Note::from_name(Diatonic::B),]
        );

        Ok(())
    }

    #[test]
    fn duration() -> Result<()> {
        let chord = parse_chord("{ a1*2, b1*4 }")?;

        assert_eq!(chord.duration, Duration::new(4, 1));

        Ok(())
    }

    #[test]
    fn same_duration() -> Result<()> {
        let chord = parse_chord("{ a1*2, b1 }")?;

        assert_eq!(
            chord.notes,
            vec![
                Note::from_name(Diatonic::A).with_duration(2, 1),
                Note::from_name(Diatonic::B).with_duration(2, 1),
            ]
        );

        Ok(())
    }

    #[test]
    fn same_octave() -> Result<()> {
        let chord = parse_chord("{ a2, b }")?;

        assert_eq!(
            chord.notes,
            vec![
                Note::from_name(Diatonic::A).with_octave(2),
                Note::from_name(Diatonic::B).with_octave(2),
            ]
        );

        Ok(())
    }
}
