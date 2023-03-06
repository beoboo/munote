use std::any::Any;

use nom::{
    character::complete::char,
    IResult,
    sequence::delimited,
};
use nom::sequence::terminated;

use crate::{
    context::ContextPtr,
    duration::Duration,
    models::ws,
    symbol::Symbol,
};
use crate::symbol::{parse_symbols, same_symbols};

#[derive(Clone, Debug)]
pub struct Chord {
    pub symbols: Vec<Box<dyn Symbol>>,
    pub duration: Duration,
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
            && same_symbols(&self.symbols, &other.symbols)
    }
}

impl Symbol for Chord {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Symbol) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn clone_box(&self) -> Box<dyn Symbol> {
        Box::new((*self).clone())
    }
}

impl Chord {
    pub fn new(symbols: Vec<Box<dyn Symbol>>, duration: Duration) -> Self {
        Self { symbols, duration }
    }

    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, symbols) = delimited(
            terminated(char('{'), ws),
            |i| parse_symbols(i, context.clone()),
            terminated(char('}'), ws),
        )(input)?;

        let ctx = context.borrow();

        Ok((input, Chord::new(symbols, ctx.duration)))
    }
}
//
// fn parse_symbols(input: &str, context: ContextPtr) -> IResult<&str, Vec<Box<dyn Symbol>>> {
//     let (input, first) = Note::parse(input, context.clone())?;
//
//     let (input, mut notes) =
//         many0(preceded(comma, |i| Note::parse(i, context.clone())))(input)?;
//
//     notes.insert(0, first);
//
//     Ok((input, notes))
// }

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::note::Note;
    use crate::note::Diatonic;
    use crate::tag::{Tag, TagId, TagParam};

    use super::*;

    fn parse_chord(input: &str) -> Result<Chord> {
        let context = ContextPtr::default();

        println!("Here");
        let (input, chord) =
            Chord::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(input, "");

        Ok(chord)
    }

    #[test]
    fn parse_one() -> Result<()> {
        let chord = parse_chord("{ a1 }")?;

        assert_same_symbols(chord.symbols, vec![Box::new(Note::from_name(Diatonic::A))]);

        Ok(())
    }

    #[test]
    fn parse_multiple() -> Result<()> {
        let chord = parse_chord("{ a1, b1 }")?;

        assert_same_symbols(
            chord.symbols,
            vec![Box::new(Note::from_name(Diatonic::A)), Box::new(Note::from_name(Diatonic::B))],
        );

        Ok(())
    }

    #[test]
    fn duration() -> Result<()> {
        let chord = parse_chord("{ a1*2, b1*4 }")?;

        println!("{chord:?}");

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
                Box::new(Tag::from_id(TagId::Staff).with_param(TagParam::Number(1.0))),
                Box::new(Note::from_name(Diatonic::G)),
            ],
        );

        Ok(())
    }

    fn assert_same_symbols(lhs: Vec<Box<dyn Symbol>>, rhs: Vec<Box<dyn Symbol>>) {
        assert_eq!(lhs.len(), rhs.len());

        for (i, s) in lhs.iter().enumerate() {
            assert_eq!(s.as_ref(), rhs[i].as_ref());
        }
    }
}
