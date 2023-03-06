use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res, opt, value, verify};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::Tuple;
use nom::{character::complete::char, sequence::preceded, IResult};
use parse_display::Display;
use parse_display::FromStr;
use strum::EnumIter;

use crate::models::{comma, ws};
use crate::symbol::parse_symbols;
use crate::{context::ContextPtr, symbol::Symbol};

#[derive(Debug)]
pub struct Tag {
    pub id: TagId,
    pub params: Vec<TagParam>,
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl Tag {
    pub fn new(id: TagId, params: Vec<TagParam>, symbols: Vec<Box<dyn Symbol>>) -> Self {
        Self { id, params, symbols }
    }

    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
        let (input, id) = map_res(preceded(char('\\'), alpha1), TagId::from_str)(input)?;

        let (input, maybe_params) = opt(delimited(char('<'), |s| parse_params(s, context.clone()), char('>')))(input)?;

        let (input, maybe_symbols) = opt(delimited(
            char('('),
            delimited(ws, |s| parse_symbols(s, context.clone()), ws),
            char(')'),
        ))(input)?;

        Ok((
            input,
            Tag::new(id, maybe_params.unwrap_or_default(), maybe_symbols.unwrap_or_default()),
        ))
    }
}

fn parse_params(input: &str, context: ContextPtr) -> IResult<&str, Vec<TagParam>> {
    let (input, first) = TagParam::parse(input, context.clone())?;

    let (input, mut params) = many0(preceded(comma, |i| TagParam::parse(i, context.clone())))(input)?;

    params.insert(0, first);

    Ok((input, params))
}

#[derive(Debug, PartialEq, FromStr)]
#[display(style = "camelCase")]
pub enum TagId {
    Accol,
    Bar,
    Meter,
    PageFormat,
    Space,
    SystemFormat,
    Staff,
    Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumIter)]
#[display(style = "camelCase")]
pub enum Unit {
    M,
    Cm,
    Mm,
    In,
    Pt,
    Pc,
    Hs,
}

#[derive(Debug, PartialEq)]
pub enum TagParam {
    Number(f32),
    NumberUnit(f32, Unit),
    String(String),
    VarNumberUnit(String, f32, Unit),
    VarString(String, String),
}

impl TagParam {
    pub fn parse(input: &str, mut context: ContextPtr) -> IResult<&str, Self> {
        alt((
            map(
                |s| parse_var_string(s, context.clone()),
                |(v, s)| TagParam::VarString(v, s),
            ),
            map(
                |s| parse_var_number_unit(s, context.clone()),
                |(v, n, u)| TagParam::VarNumberUnit(v, n, u),
            ),
            map(|s| parse_string(s, context.clone()), TagParam::String),
            map(
                |s| parse_number_unit(s, context.clone()),
                |(n, u)| TagParam::NumberUnit(n, u),
            ),
            map(|s| parse_number(s, context.clone()), TagParam::Number),
        ))(input)
    }
}

fn parse_string(input: &str, _context: ContextPtr) -> IResult<&str, String> {
    let not_quote_slash = is_not("\"\\");

    let check_string = verify(not_quote_slash, |s: &str| !s.is_empty());

    let (input, s) = delimited(char('"'), check_string, char('"'))(input)?;

    Ok((input, s.to_string()))
}

fn parse_number(input: &str, _context: ContextPtr) -> IResult<&str, f32> {
    map_res(digit1, f32::from_str)(input)
}

fn parse_unit(input: &str, _context: ContextPtr) -> IResult<&str, Unit> {
    alt((
        value(Unit::Mm, tag("mm")),
        value(Unit::M, tag("m")),
        value(Unit::Cm, tag("cm")),
        value(Unit::In, tag("in")),
        value(Unit::Pt, tag("pt")),
        value(Unit::Pc, tag("pc")),
        value(Unit::Hs, tag("hs")),
    ))(input)
}

fn parse_number_unit(input: &str, context: ContextPtr) -> IResult<&str, (f32, Unit)> {
    let (input, number) = parse_number(input, context.clone())?;
    let (input, unit) = parse_unit(input, context)?;

    Ok((input, (number, unit)))
}

fn parse_var_string(input: &str, context: ContextPtr) -> IResult<&str, (String, String)> {
    let (input, (name, _, val)) = (alpha1, delimited(ws, char('='), ws), |s| {
        parse_string(s, context.clone())
    })
        .parse(input)?;

    Ok((input, (name.to_string(), val)))
}

fn parse_var_number_unit(input: &str, context: ContextPtr) -> IResult<&str, (String, f32, Unit)> {
    let (input, (name, _, (num, unit))) = (alpha1, delimited(ws, char('='), ws), |s| {
        parse_number_unit(s, context.clone())
    })
        .parse(input)?;

    Ok((input, (name.to_string(), num, unit)))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use strum::IntoEnumIterator;

    use crate::note::{Diatonic, Note};

    use super::*;

    fn parse_tag(input: &str) -> Result<Tag> {
        let context = ContextPtr::default();

        let (_, tag) = Tag::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        Ok(tag)
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

        Ok(())
    }

    #[test]
    fn parse_number_param() -> Result<()> {
        let tag = parse_tag("\\staff<1>")?;

        assert_eq!(tag.id, TagId::Staff);
        assert_eq!(tag.params, vec![TagParam::Number(1.0)]);

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
        assert_eq!(tag.params, vec![
            TagParam::VarNumberUnit("lm".into(), 1.0, Unit::Cm),
            TagParam::VarNumberUnit("tm".into(), 1.0, Unit::Cm),
            TagParam::VarNumberUnit("bm".into(),1.0, Unit::Cm),
            TagParam::VarNumberUnit("rm".into(), 1.0, Unit::Cm),
        ]);

        Ok(())
    }

    #[test]
    fn parse_variable_number_unit_param() -> Result<()> {
        let tag = parse_tag("\\systemFormat<dx=1cm>")?;

        println!("{:?}", tag);

        assert_eq!(tag.id, TagId::SystemFormat);
        assert_eq!(tag.params, vec![TagParam::VarNumberUnit("dx".into(), 1.0, Unit::Cm)]);

        Ok(())
    }

    #[test]
    fn parse_units() -> Result<()> {
        for u in Unit::iter() {
            let tag = parse_tag(&format!("\\systemFormat<dx=1{u}>"))?;
            assert_eq!(tag.params, vec![TagParam::VarNumberUnit("dx".into(), 1.0, u)]);
        }

        Ok(())
    }

    #[test]
    fn parse_variable_string_param() -> Result<()> {
        let tag = parse_tag("\\accol<type=\"thinBrace\">")?;

        assert_eq!(tag.id, TagId::Accol);
        assert_eq!(tag.params, vec![TagParam::VarString("type".into(), "thinBrace".into())]);

        Ok(())
    }

    #[test]
    fn parse_symbols() -> Result<()> {
        let tag = parse_tag("\\tie(d e)>")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.symbols.len(), 2);

        assert!(tag.symbols[0].equals(&Note::from_name(Diatonic::D)));
        assert!(tag.symbols[1].equals(&Note::from_name(Diatonic::E)));

        Ok(())
    }
}
