use nom::{
    character::complete::{char, one_of},
    combinator::peek,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{chord::Chord, context::ContextPtr, models::ws, note::Note, rest::Rest, symbol::Symbol};

#[derive(Debug)]
pub struct Voice {
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl Voice {
    pub fn new(symbols: Vec<Box<dyn Symbol>>) -> Self {
        Self { symbols }
    }

    pub fn parse<'a>(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, symbols) = delimited(
            char('['),
            delimited(ws, |s| parse_symbols(s, context.clone()), ws),
            char(']'),
        )(input)?;

        Ok((input, Voice::new(symbols)))
    }
}

fn parse_symbols(input: &str, context: ContextPtr) -> IResult<&str, Vec<Box<dyn Symbol>>> {
    let (input, first) = parse(input)?;

    // Ok((input, vec![first]))

    let (input, mut symbols) = many0(preceded(ws, |i| parse_next(i, context.clone())))(input)?;

    symbols.insert(0, first);

    Ok((input, symbols))
}

fn parse(input: &str) -> IResult<&str, Box<dyn Symbol>> {
    let context = ContextPtr::default();
    parse_next(input, context)
}

fn parse_next(input: &str, context: ContextPtr) -> IResult<&str, Box<dyn Symbol>> {
    let (_, next) = peek(one_of("abcdefghilmrst{_"))(input)?;

    let (input, symbol) = match next {
        '{' => {
            let (input, chord) = Chord::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(chord);
            (input, b)
        },
        '_' => {
            let (input, rest) = Rest::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(rest);
            (input, b)
        },
        _ => {
            let (input, note) = Note::parse(input, context)?;
            let b: Box<dyn Symbol> = Box::new(note);
            (input, b)
        },
    };

    Ok((input, symbol))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::{chord::Chord, note::Diatonic, rest::Rest};

    use super::*;

    fn parse_voice(input: &str) -> Result<Voice> {
        let context = ContextPtr::default();

        let (_, voice) = Voice::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        Ok(voice)
    }

    #[test]
    fn parse_one_note() -> Result<()> {
        let voice = parse_voice("[ a1 ]")?;

        assert!(voice.symbols[0].equals(&Note::from_name(Diatonic::A)));

        Ok(())
    }

    #[test]
    fn parse_notes_and_rests() -> Result<()> {
        let voice = parse_voice("[ a1 _ ]")?;

        assert_eq!(voice.symbols.len(), 2);

        assert!(voice.symbols[0].equals(&Note::from_name(Diatonic::A)));
        assert!(voice.symbols[1].equals(&Rest::default()));

        Ok(())
    }

    #[test]
    fn parse_chord() -> Result<()> {
        let voice = parse_voice("[ { a1*2, b } ]")?;

        assert_eq!(voice.symbols.len(), 1);

        let chord = Chord::new(vec![
            Note::from_name(Diatonic::A).with_duration(2, 1),
            Note::from_name(Diatonic::B).with_duration(2, 1),
        ]);

        assert!(voice.symbols[0].equals(&chord));

        Ok(())
    }
}
