use std::convert::From;
use std::str::FromStr;

use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

use crate::duration::Duration;
use crate::models::{comma, ws};
use crate::note::Note;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: Duration,
}

impl Chord {
    pub fn new(notes: Vec<Note>, duration: Duration) -> Self {
        Self { notes, duration }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, notes) = delimited(char('{'), delimited(ws, parse_notes, ws), char('}'))(input)?;
        let duration = notes.iter().map(|n| n.duration()).max();

        Ok((input, Chord::new(notes, duration.unwrap())))
    }
}

fn parse_notes(input: &str) -> IResult<&str, Vec<Note>> {
    let (input, first) = Note::parse(input)?;

    let (input, mut notes) = many0(preceded(comma, |i| {
        Note::parse_next(i, first.octave(), first.duration())
    }))(input)?;

    notes.insert(0, first);

    Ok((input, notes))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::note::Diatonic;

    use super::*;

    #[test]
    fn parse_one() -> Result<()> {
        let (_, chord) = Chord::parse("{ a1 }")?;

        assert_eq!(chord.notes, vec![Note::from_name(Diatonic::A)]);

        Ok(())
    }

    #[test]
    fn parse_multiple() -> Result<()> {
        let (_, chord) = Chord::parse("{ a1, b1 }")?;

        assert_eq!(
            chord.notes,
            vec![Note::from_name(Diatonic::A), Note::from_name(Diatonic::B),]
        );

        Ok(())
    }

    #[test]
    fn duration() -> Result<()> {
        let (_, chord) = Chord::parse("{ a1*2, b1*4 }")?;

        assert_eq!(chord.duration, Duration::new(4, 1));

        Ok(())
    }

    #[test]
    fn same_duration() -> Result<()> {
        let (_, chord) = Chord::parse("{ a1*2, b1 }")?;

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
        let (_, chord) = Chord::parse("{ a2, b }")?;

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
