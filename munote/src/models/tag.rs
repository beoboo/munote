use std::any::Any;

use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, char}, combinator::opt, error_position, IResult, multi::many0, sequence::{delimited, preceded, terminated}};
use nom::Err;
use nom::error::ErrorKind;

use crate::{
    context::ContextPtr,
    impl_symbol_for,
    models::ws,
    event::{same_symbols, Event},
    tag_id::TagId,
    tag_param::TagParam,
};
use crate::event::parse_delimited_events;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub params: Vec<TagParam>,
    pub events: Vec<Box<dyn Event>>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && same_symbols(&self.events, &other.events)
    }
}

impl_symbol_for!(Tag);

impl Tag {
    pub fn new(
        id: TagId,
        params: Vec<TagParam>,
        events: Vec<Box<dyn Event>>,
    ) -> Self {
        Self {
            id,
            params,
            events,
        }
    }

    pub fn from_id(id: TagId) -> Self {
        Self::new(id, Vec::new(), Vec::new())
    }

    pub fn with_param(mut self, param: TagParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_symbol(mut self, symbol: Box<dyn Event>) -> Self {
        self.events.push(symbol);
        self
    }

    pub fn parse(input: &str, mut context: ContextPtr) -> IResult<&str, Option<Self>> {
        // println!("\n\n\nParsing \"{input}\"\n\n");
        let (input, maybe_id) =
            alt((terminated(preceded(char('\\'), alpha1), ws), tag("|"))
            )(input)?;

        let (skip_start_delimiter, name) = if maybe_id.ends_with("Begin") {
            (true, maybe_id.replace("Begin", ""))
        } else if maybe_id.ends_with("End") {
            return Ok((input, None));
        } else {
            (false, maybe_id.to_string())
        };

        let id = TagId::lookup(&name)
            .map_err(|_| Err::Error(error_position!(input, ErrorKind::Fail)))?;

        // println!("\n\n\nParsing \"{input}\" for {id:?}");

        let (input, maybe_params) = opt(delimited(
            terminated(char('<'), ws),
            |s| parse_params(s),
            terminated(char('>'), ws),
        ))(input)?;
        // println!("\n\n\nParsing \"{input}\" for {id:?}{maybe_params:?}");

        let (input, maybe_symbols) = opt(
            |s| parse_delimited_events(s, context.clone(), '(', ')', skip_start_delimiter),
        )(input)?;
        // println!("\n\nParsing \"{input}\" for {id:?}
        //   params: {maybe_params:?}
        //   symbols: {maybe_symbols:?}
        //  \n");

        let tag = Tag::new(
            id,
            maybe_params.unwrap_or_default(),
            maybe_symbols.unwrap_or_default(),
        );

        let mut context = context.borrow_mut();
        // context.add_tag(tag.clone());

        // println!("\n\n\nParsed \"{tag:?}\"");
        Ok((input, Some(tag)))
    }

    pub fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    pub fn as_number(&self) -> Option<f32> {
        if !self.has_params() {
            return None;
        }

        if let TagParam::Number(n) = self.params[0] {
            Some(n)
        } else {
            None
        }
    }
}

fn parse_params(input: &str) -> IResult<&str, Vec<TagParam>> {
    // println!("Parsing params: {input}");
    let (input, first) = TagParam::parse(input)?;
    // println!("First: {first:?} {input}");

    let (input, mut params) = many0(preceded(
        terminated(char(','), ws),
        |i| TagParam::parse(i),
    ))(input)?;

    params.insert(0, first);

    Ok((input, params))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use strum::IntoEnumIterator;

    use crate::{
        accidentals::Accidentals,
        note::{Diatonic, Note},
        unit::Unit,
    };

    use super::*;

    fn parse_tag(input: &str) -> Result<Tag> {
        let context = ContextPtr::default();

        let (input, tag) =
            Tag::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(input, "");

        Ok(tag.unwrap())
    }

    #[test]
    fn parse_simple() -> Result<()> {
        let tag = parse_tag("\\bar")?;

        assert_eq!(tag.id, TagId::Bar);

        Ok(())
    }

    #[test]
    fn parse_string_param() -> Result<()> {
        let tag = parse_tag("\\meter<\"2/4\">")?;

        assert_eq!(tag.id, TagId::Meter);
        assert_eq!(tag.params, vec![TagParam::String("2/4".into())]);

        let tag = parse_tag("\\harmony<\"F#m7\">")?;

        assert_eq!(tag.id, TagId::Harmony);
        assert_eq!(tag.params, vec![TagParam::String("F#m7".into())]);

        Ok(())
    }

    #[test]
    fn parse_int_number_param() -> Result<()> {
        let tag = parse_tag("\\staff<1>")?;

        assert_eq!(tag.id, TagId::Staff);
        assert_eq!(tag.params, vec![TagParam::Number(1.0)]);

        Ok(())
    }

    #[test]
    fn parse_float_number_param() -> Result<()> {
        let tag = parse_tag("\\accidental<dy=-0.5hs>")?;

        assert_eq!(tag.id, TagId::Accidental);
        assert_eq!(
            tag.params,
            vec![TagParam::VarNumberUnit("dy".into(), -0.5, Unit::Hs)]
        );

        Ok(())
    }

    #[test]
    fn parse_number_unit_param() -> Result<()> {
        let tag = parse_tag("\\space<1cm>")?;

        assert_eq!(tag.id, TagId::Space);
        assert_eq!(tag.params, vec![TagParam::NumberUnit(1.0, Unit::Cm)]);

        Ok(())
    }

    #[test]
    fn parse_param_list() -> Result<()> {
        let tag = parse_tag("\\pageFormat<lm=1cm, tm=1cm, bm=1cm, rm=1cm>")?;

        assert_eq!(tag.id, TagId::PageFormat);
        assert_eq!(
            tag.params,
            vec![
                TagParam::VarNumberUnit("lm".into(), 1.0, Unit::Cm),
                TagParam::VarNumberUnit("tm".into(), 1.0, Unit::Cm),
                TagParam::VarNumberUnit("bm".into(), 1.0, Unit::Cm),
                TagParam::VarNumberUnit("rm".into(), 1.0, Unit::Cm),
            ]
        );

        Ok(())
    }

    #[test]
    fn parse_variable_number_unit_param() -> Result<()> {
        let tag = parse_tag("\\systemFormat<dx=1cm>")?;

        println!("{:?}", tag);

        assert_eq!(tag.id, TagId::SystemFormat);
        assert_eq!(
            tag.params,
            vec![TagParam::VarNumberUnit("dx".into(), 1.0, Unit::Cm)]
        );

        Ok(())
    }

    #[test]
    fn parse_units() -> Result<()> {
        for u in Unit::iter() {
            let tag = parse_tag(&format!("\\systemFormat<dx=1{u}>"))?;
            assert_eq!(
                tag.params,
                vec![TagParam::VarNumberUnit("dx".into(), 1.0, u)]
            );
        }

        Ok(())
    }

    #[test]
    fn parse_variable_string_param() -> Result<()> {
        let tag = parse_tag("\\accol<type=\"thinBrace\">")?;

        assert_eq!(tag.id, TagId::Accolade);
        assert_eq!(
            tag.params,
            vec![TagParam::VarString("type".into(), "thinBrace".into())]
        );

        Ok(())
    }

    #[test]
    fn parse_symbols() -> Result<()> {
        let tag = parse_tag("\\tie(d e)")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.events.len(), 2);

        assert!(tag.events[0].equals(&Note::from_name(Diatonic::D)));
        assert!(tag.events[1].equals(&Note::from_name(Diatonic::E)));

        Ok(())
    }

    #[test]
    fn as_number() -> Result<()> {
        let tag = parse_tag("\\staff<1>")?;

        assert_eq!(tag.as_number().unwrap(), 1.0);

        Ok(())
    }

    #[test]
    fn compound() -> Result<()> {
        let tag = parse_tag("\\accidental<size=1.4>(d&)")?;

        assert_eq!(tag.id, TagId::Accidental);
        assert_eq!(tag.params, vec![TagParam::VarNumber("size".into(), 1.4)]);
        assert!(tag.events[0].equals(
            &Note::from_name(Diatonic::D).with_accidentals(Accidentals::Flat)
        ));

        assert_tag_id(
            parse_tag("\\tuplet<\"-3-\",dy1=-3, dy2=1>(c/6 d e&)")?,
            TagId::Tuplet,
        );

        assert_tag_id(
            parse_tag("\\tie (a/1 | \\harmony<\"G7\", dy=2> a)")?,
            TagId::Tie,
        );

        assert_tag_id(
            parse_tag("\\instr<\"Pizz.\",  autopos=\"on\", fsize=10pt>")?,
            TagId::Instrument,
        );

        assert_tag_id(
            parse_tag("\\pizz<\"buzz\"> (\\stacc(a1 b) \\ten(a1 b))")?,
            TagId::Pizzicato,
        );

        assert_tag_id(
            parse_tag(
                "\\text<\"dolce\", dy=13, fattrib=\"i\", fsize=10pt>(g/4)",
            )?,
            TagId::Text,
        );

        assert_tag_id(
            parse_tag(
                "\\lyrics<\"Dans un som-meil  que char mait ton i-ma-ge\">(\\dim<dy=15, deltaY=2>(c/2 b1/4))",
            )?,
            TagId::Lyrics,
        );

        Ok(())
    }

    #[test]
    fn eat_whitespaces() -> Result<()> {
        assert_tag_id(
            parse_tag("\\tuplet < \"-3-\",dy1=-3, dy2=1>(c/6 d e&)")?,
            TagId::Tuplet,
        );

        Ok(())
    }

    #[test]
    fn lookup_variants() -> Result<()> {
        assert_tag_id(parse_tag("\\acc")?, TagId::Accidental);
        assert_tag_id(parse_tag("|")?, TagId::Bar);

        Ok(())
    }

    #[test]
    fn nested_tag() -> Result<()> {
        let tag = parse_tag("\\lyrics<\"Hi\">(\\text<\"there\">(a))")?;
        let expected: Box<dyn Event> = Box::new(
            Tag::from_id(TagId::Text)
                .with_param(TagParam::String("there".into()))
                .with_symbol(Box::new(Note::from_name(Diatonic::A))),
        );

        assert_eq!(tag.id, TagId::Lyrics);
        assert_eq!(&tag.events[0], &expected);

        Ok(())
    }

    fn assert_tag_id(tag: Tag, expected: TagId) {
        assert_eq!(tag.id, expected)
    }
}
