use std::convert::From;
use std::str::FromStr;

use anyhow::Result;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_while, take_while_m_n};
use nom::character::complete::{alpha1, char as ch, digit1, i8, one_of, u8};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use parse_display::FromStr;

pub struct NoteParser;

impl NoteParser {
    pub fn parse_note(&self, input: impl Into<String>) -> Result<Note> {
        Ok(Note::new(NoteName::Diatonic(Diatonic::A), vec![], 1, Duration::default(), 0))
    }
}

#[derive(Debug, Clone, FromStr, PartialEq)]
#[display(style = "lowercase")]
enum Diatonic {
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
    fn parse(input: &str) -> IResult<&str, Self> {
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
enum Chromatic {
    Cis,
    Dis,
    Fis,
    Gis,
    Ais,
}

impl Chromatic {
    fn parse(input: &str) -> IResult<&str, Self> {
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
enum Solfege {
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
    fn parse(input: &str) -> IResult<&str, Self> {
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
enum NoteName {
    Diatonic(Diatonic),
    Chromatic(Chromatic),
    Solfege(Solfege),
}

#[derive(Debug, Clone, PartialEq)]
struct Duration {
    num: u8,
    denom: u8,
}

impl Default for Duration {
    fn default() -> Self {
        Self { num: 1, denom: 1 }
    }
}

impl Duration {
    fn new(num: u8, denom: u8) -> Self {
        Self { num, denom }
    }
}

fn accidentals(input: &str) -> IResult<&str, Vec<char>> {
    let (input, accs) = is_a("&#")(input)?;
    let accs = accs.chars().collect();

    Ok((input, accs))
}

fn duration_num(input: &str) -> IResult<&str, u8> {
    let (input, _) = ch('*')(input)?;
    u8(input)
}

fn duration_denom(input: &str) -> IResult<&str, u8> {
    let (input, _) = ch('/')(input)?;
    u8(input)
}

fn duration(input: &str) -> IResult<&str, Duration> {
    let (input, num) = duration_num(input).unwrap_or((input, 1));
    let (input, denom) = duration_denom(input).unwrap_or((input, 1));
    let (input, _) = opt(tag("ms"))(input)?;

    Ok((input, Duration { num, denom }))
}

struct Note {
    name: NoteName,
    octave: i8,
    accidentals: Vec<char>,
    duration: Duration,
    dots: u8,
}

impl Note {
    pub fn new(
        name: impl Into<NoteName>,
        accidentals: Vec<char>,
        octave: i8,
        duration: Duration,
        dots: u8,
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
        )).parse(input)?;

        let (input, maybe_accidentals) = opt(accidentals).parse(input)?;
        let (input, maybe_octave) = opt(i8).parse(input)?;
        let (input, maybe_duration) = opt(duration).parse(input)?;
        let (input, dots) = take_while_m_n(0, 3, |c| c == '.').parse(input)?;

        Ok((input, Note::new(
            name,
            maybe_accidentals.unwrap_or(Vec::new()),
            maybe_octave.unwrap_or(1),
            maybe_duration.unwrap_or_default(),
            dots.len() as u8
        )))
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(note.accidentals, vec!['#']);

        Ok(())
    }

    #[test]
    fn parse_duration() -> Result<()> {
        let (_, note) = Note::parse("c*1")?;
        assert_eq!(note.duration, Duration::new(1, 1));

        let (_, note) = Note::parse("c*2/4")?;
        assert_eq!(note.duration, Duration::new(2, 4));

        let (_, note) = Note::parse("c/3")?;
        assert_eq!(note.duration, Duration::new(1, 3));

        let (_, note) = Note::parse("c*5ms")?;
        assert_eq!(note.duration, Duration::new(5, 1));

        Ok(())
    }

    #[test]
    fn parse_dots() -> Result<()> {
        let (_, note) = Note::parse("c")?;
        assert_eq!(note.dots, 0);

        let (_, note) = Note::parse("c..")?;
        assert_eq!(note.dots, 2);

        let (_, note) = Note::parse("c...")?;
        assert_eq!(note.dots, 3);

        Ok(())
    }

    fn assert_note(note: &Note, name: impl Into<NoteName>, octave: i8) {
        assert_eq!(note.name, name.into());
        assert_eq!(note.octave, octave);
    }
}