use nom::IResult;

use crate::{
    context::ContextPtr,
    event::Event,
    tag::Tag,
    tag_id::TagId,
};
use crate::event::parse_delimited_events;

#[derive(Debug)]
pub struct Voice {
    pub staff: u8,
    pub events: Vec<Box<dyn Event>>,
    // pub range_tags: Vec<RangeTag>,
}

impl Voice {
    pub fn new(staff: u8, events: Vec<Box<dyn Event>>) -> Self {
        Self { staff, events }
    }

    pub fn parse<'a>(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, events) = parse_delimited_events(input, context.clone(), '[', ']', false)?;

        let ctx = context.borrow();
        let staff = None;//ctx.get_tag(TagId::Staff).and_then(Tag::as_number);

        Ok((input, Voice::new(staff.unwrap_or(1.0) as u8, events)))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use crate::{
        chord::Chord,
        duration::Duration,
        note::{Diatonic, Note},
        rest::Rest,
    };

    use super::*;

    fn parse_voice(input: &str) -> Result<Voice> {
        let context = ContextPtr::default();

        let (input, voice) =
            Voice::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(input, "");

        Ok(voice)
    }

    #[test]
    fn parse_one_note() -> Result<()> {
        let voice = parse_voice("[ a1 ]")?;

        assert!(voice.events[0].equals(&Note::from_name(Diatonic::A)));

        Ok(())
    }

    #[test]
    fn parse_notes_and_rests() -> Result<()> {
        let voice = parse_voice("[ a1 _ ]")?;

        assert_eq!(voice.events.len(), 2);

        assert!(voice.events[0].equals(&Note::from_name(Diatonic::A)));
        assert!(voice.events[1].equals(&Rest::default()));

        Ok(())
    }

    #[test]
    fn parse_chord() -> Result<()> {
        let voice = parse_voice("[ { a1*2, b } ]")?;

        assert_eq!(voice.events.len(), 1);

        let chord = Chord::new(
            vec![
                Box::new(Note::from_name(Diatonic::A).with_duration(2, 1)),
                Box::new(Note::from_name(Diatonic::B).with_duration(2, 1)),
            ],
            Duration::new(2, 1),
        );

        println!("{chord:?}");
        println!("{:?}", voice.events[0]);
        assert!(voice.events[0].equals(&chord));

        Ok(())
    }

    #[test]
    fn parse_tag() -> Result<()> {
        let voice = parse_voice("[ \\meter<\"2/4\"> a1 ]")?;

        assert_eq!(voice.events.len(), 2);

        Ok(())
    }

    #[test]
    fn convert_begin_end() -> Result<()> {
        let voice = parse_voice("[ \\tieBegin d e \\tieEnd ]")?;
        assert_eq!(voice.events.len(), 2);
        todo!();
        // assert_eq!(voice.range_tags.len(), 1);
        // assert_eq!(voice.range_tags[0], RangeTag::from_id(TagId::Tie).with_events(vec![
        //     Box::new(Note::from_name(Diatonic::D)),
        //     Box::new(Note::from_name(Diatonic::E)),
        // ]));

        // assert!(tag.symbols[0].equals(&Note::from_name(Diatonic::D)));
        // assert!(tag.symbols[1].equals(&Note::from_name(Diatonic::E)));

        Ok(())
    }

}
