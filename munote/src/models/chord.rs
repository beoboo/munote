use std::any::Any;

use nom::IResult;

use crate::{
    context::ContextPtr,
    duration::Duration,
    impl_symbol_for,
    event::{same_symbols, Event},
};
use crate::event::parse_delimited_events;

#[derive(Clone, Debug)]
pub struct Chord {
    pub symbols: Vec<Box<dyn Event>>,
    pub duration: Duration,
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
            && same_symbols(&self.symbols, &other.symbols)
    }
}

impl_symbol_for!(Chord);

impl Chord {
    pub fn new(symbols: Vec<Box<dyn Event>>, duration: Duration) -> Self {
        Self { symbols, duration }
    }

    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, symbols) = parse_delimited_events(input, context.clone(), '{', '}', false)?;

        let ctx = context.borrow();

        Ok((input, Chord::new(symbols, ctx.duration)))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::{
        note::{Diatonic, Note},
        tag::Tag,
        tag_id::TagId,
        tag_param::TagParam,
    };

    use super::*;

    fn parse_chord(input: &str) -> Result<Chord> {
        let context = ContextPtr::default();

        let (input, chord) =
            Chord::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(input, "");

        Ok(chord)
    }

    #[test]
    fn parse_one() -> Result<()> {
        let chord = parse_chord("{ a1 }")?;

        assert_same_symbols(
            chord.symbols,
            vec![Box::new(Note::from_name(Diatonic::A))],
        );

        Ok(())
    }

    #[test]
    fn parse_multiple() -> Result<()> {
        let chord = parse_chord("{ a1, b1 }")?;

        assert_same_symbols(
            chord.symbols,
            vec![
                Box::new(Note::from_name(Diatonic::A)),
                Box::new(Note::from_name(Diatonic::B)),
            ],
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

        assert_same_symbols(
            chord.symbols,
            vec![
                Box::new(Note::from_name(Diatonic::A).with_duration(2, 1)),
                Box::new(Note::from_name(Diatonic::B).with_duration(2, 1)),
            ],
        );

        Ok(())
    }

    #[test]
    fn same_octave() -> Result<()> {
        let chord = parse_chord("{ a2, b }")?;

        assert_same_symbols(
            chord.symbols,
            vec![
                Box::new(Note::from_name(Diatonic::A).with_octave(2)),
                Box::new(Note::from_name(Diatonic::B).with_octave(2)),
            ],
        );

        Ok(())
    }

    #[test]
    fn with_tags() -> Result<()> {
        let chord = parse_chord("{ c, \\staff<1> g }")?;
        println!("Here");

        assert_same_symbols(
            chord.symbols,
            vec![
                Box::new(Note::from_name(Diatonic::C)),
                Box::new(
                    Tag::from_id(TagId::Staff)
                        .with_param(TagParam::Number(1.0)),
                ),
                Box::new(Note::from_name(Diatonic::G)),
            ],
        );

        Ok(())
    }

    fn assert_same_symbols(
        lhs: Vec<Box<dyn Event>>,
        rhs: Vec<Box<dyn Event>>,
    ) {
        assert_eq!(lhs.len(), rhs.len());

        for (i, s) in lhs.iter().enumerate() {
            assert_eq!(s.as_ref(), rhs[i].as_ref());
        }
    }
}
