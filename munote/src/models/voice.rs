use nom::{character::complete::char, sequence::delimited, IResult};

use crate::{
    context::ContextPtr,
    models::ws,
    symbol::{parse_symbols, Symbol},
    tag::{Tag, TagId},
};

#[derive(Debug)]
pub struct Voice {
    pub staff: u8,
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl Voice {
    pub fn new(staff: u8, symbols: Vec<Box<dyn Symbol>>) -> Self {
        Self { staff, symbols }
    }

    pub fn parse<'a>(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, symbols) = delimited(
            char('['),
            delimited(ws, |s| parse_symbols(s, context.clone()), ws),
            char(']'),
        )(input)?;

        let ctx = context.borrow();
        let staff = ctx.get_tag(TagId::Staff).and_then(Tag::as_number);

        Ok((input, Voice::new(staff.unwrap_or(1.0) as u8, symbols)))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::{
        chord::Chord,
        note::{Diatonic, Note},
        rest::Rest,
    };

    use super::*;

    fn parse_voice(input: &str) -> Result<Voice> {
        let context = ContextPtr::default();

        let (input, voice) =
            Voice::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert!(input.is_empty());

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

    #[test]
    fn parse_tag() -> Result<()> {
        let voice = parse_voice("[ \\meter<\"2/4\"> a1 ]")?;

        assert_eq!(voice.symbols.len(), 2);

        Ok(())
    }
}
