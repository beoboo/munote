use std::any::Any;
use std::str::FromStr;

use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, char}, combinator::opt, error_position, IResult, multi::many0, sequence::{delimited, preceded, terminated}};
use nom::character::complete::u8;
use nom::Err;
use nom::error::ErrorKind;
use serde::Deserialize;

use crate::{
    context::ContextPtr,
    event::Event,
    impl_symbol_for,
    models::ws,
    tag_id::TagId,
    tag_param::TagParam,
};
use crate::event::parse_delimited_events;
use crate::models::Span;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TagType {
    Any,
    Position,
    Begin(u8),
    End(u8),
    Range,
}

impl PartialEq for TagType {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            TagType::Position => matches!(other, TagType::Position | TagType::Any),
            _ => *other != TagType::Position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: TagId,
    pub ty: TagType,
    pub params: Vec<TagParam>,
    pub events: Vec<Box<dyn Event>>,
}

impl_symbol_for!(Tag);

impl Tag {
    pub fn new(
        id: TagId,
        ty: TagType,
        params: Vec<TagParam>,
        events: Vec<Box<dyn Event>>,
    ) -> Self {
        Self {
            id,
            ty,
            params,
            events,
        }
    }

    pub fn from_id(id: TagId) -> Self {
        Self::new(id, TagType::Position, Vec::new(), Vec::new())
    }

    pub fn with_type(mut self, ty: TagType) -> Self {
        self.ty = ty;
        self
    }

    pub fn with_param(mut self, param: TagParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_event(mut self, event: Box<dyn Event>) -> Self {
        self.events.push(event);
        self
    }

    pub fn parse(input: Span, mut context: ContextPtr) -> IResult<Span, Self> {
        // println!("\n\n\nParsing \"{input}\"\n\n");
        let (input, maybe_id) =
            alt((terminated(preceded(char('\\'), alpha1), ws), tag("|"))
            )(input)?;

        let maybe_id = maybe_id.fragment();
        let (input, suffix) = parse_suffix(input).unwrap_or_else(|_| (input, 0));

        let (input, maybe_params) = opt(delimited(
            terminated(char('<'), ws),
            |s| parse_params(s),
            terminated(char('>'), ws),
        ))(input)?;
        // println!("\n\n\nParsing \"{input}\" for {id:?}{maybe_params:?}");

        let (input, maybe_events) = opt(
            |s| parse_delimited_events(s, context.clone(), '(', ')'),
        )(input)?;

        let mut ctx = context.borrow_mut();

        // TODO: revisit this, its awful like this
        let mut ty = TagType::Position;
        let maybe_id = if maybe_id.ends_with("Begin") {
            if TagId::from_str(maybe_id).is_ok() {
                maybe_id.to_string()
            } else {
                ty = TagType::Begin(suffix);
                maybe_id.replace("Begin", "")
            }
        } else if maybe_id.ends_with("End") {
            if TagId::from_str(maybe_id).is_ok() {
                maybe_id.to_string()
            } else {
                ty = TagType::End(suffix);
                maybe_id.replace("End", "")
            }
        } else {
            if maybe_events.is_some() {
                ty = TagType::Range;
            };

            maybe_id.to_string()
        };

        let id = ctx.lookup_tag(&maybe_id)
            .map_err(|e| {
                println!("{e:?}");
                Err::Error(error_position!(input, ErrorKind::Fail))
            })?;

        // println!("\n\nParsing \"{input}\" for {id:?} {ty:?}
        //   params: {maybe_params:?}
        //   events: {maybe_events:?}
        //  \n");
        //
        let tag = Tag::new(
            id,
            ty,
            maybe_params.unwrap_or_default(),
            maybe_events.unwrap_or_default(),
        );

        if let Err(e) = ctx.validate(&tag) {
            println!("Could not validate \"{:?}\": {e}", tag.id);

            return Err(Err::Error(error_position!(input, ErrorKind::Fail)));
        }

        ctx.add_tag(tag.clone());

        Ok((input, tag))
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

fn parse_suffix(input: Span) -> IResult<Span, u8> {
    preceded(char(':'), u8)(input)
}

fn parse_params(input: Span) -> IResult<Span, Vec<TagParam>> {
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

    use crate::{
        accidentals::Accidentals,
        note::{Diatonic, Note},
        unit::Unit,
    };

    use super::*;

    fn parse_tag(input: &str) -> Result<Tag> {
        let context = ContextPtr::default();

        let (input, parsed) =
            Tag::parse(Span::new(input), context).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn parse_simple() -> Result<()> {
        let tag = parse_tag("\\bar")?;

        assert_eq!(tag.id, TagId::Bar);
        assert_eq!(tag.ty, TagType::Position);

        Ok(())
    }

    #[test]
    fn parse_suffix() -> Result<()> {
        let tag = parse_tag("\\tieBegin:1(a)")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.ty, TagType::Begin(1));

        Ok(())
    }

    #[test]
    fn parse_begin_end() -> Result<()> {
        let tag = parse_tag("\\tieBegin")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.ty, TagType::Begin(0));

        let tag = parse_tag("\\tieEnd")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.ty, TagType::End(0));

        Ok(())
    }

    #[test]
    fn parse_param() -> Result<()> {
        let tag = parse_tag("\\meter<\"2/4\">")?;

        assert_eq!(tag.id, TagId::Meter);
        assert_eq!(tag.params, vec![TagParam::String("2/4".into())]);

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
    fn parse_events() -> Result<()> {
        let tag = parse_tag("\\tie(d e)")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.ty, TagType::Range);
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
        assert_tag_id(parse_tag("|")?, TagId::Bar);
        assert_tag_id(parse_tag("\\acc(a)")?, TagId::Accidental);

        Ok(())
    }

    #[test]
    fn nested_tag() -> Result<()> {
        let tag = parse_tag("\\text<\"Hi\">(\\text<\"there\">)")?;
        let expected: Box<dyn Event> = Box::new(
            Tag::from_id(TagId::Text)
                .with_param(TagParam::String("there".into()))
        );

        assert_eq!(tag.id, TagId::Text);
        assert_eq!(tag.ty, TagType::Range);
        assert_eq!(&tag.events[0], &expected);

        Ok(())
    }

    fn assert_tag_id(tag: Tag, expected: TagId) {
        assert_eq!(tag.id, expected)
    }
}
