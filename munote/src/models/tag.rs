use std::any::Any;
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

use crate::duration::Duration;
use crate::models::{comma, ws};
use crate::symbol::parse_symbols;
use crate::{context::ContextPtr, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub params: Vec<TagParam>,
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.params == other.params && same_symbols(self, other)
    }
}

fn same_symbols(lhs: &Tag, rhs: &Tag) -> bool {
    lhs.symbols.len() == rhs.symbols.len()
        && lhs
            .symbols
            .iter()
            .enumerate()
            .fold(true, |val, (i, s)| val && s.equals(&*rhs.symbols[i]))
}

impl Symbol for Tag {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Symbol) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn clone_box(&self) -> Box<dyn Symbol> {
        Box::new((*self).clone())
    }

    fn octave(&self) -> i8 {
        1
    }

    fn duration(&self) -> Duration {
        Duration::default()
    }
}

impl Tag {
    pub fn new(id: TagId, params: Vec<TagParam>, symbols: Vec<Box<dyn Symbol>>) -> Self {
        Self { id, params, symbols }
    }

    pub fn from_id(id: TagId) -> Self {
        Self::new(id, Vec::new(), Vec::new())
    }

    pub fn parse(input: &str, mut context: ContextPtr) -> IResult<&str, Self> {
        let (input, id) = map_res(preceded(char('\\'), alpha1), TagId::from_str)(input)?;

        let (input, maybe_params) = opt(delimited(char('<'), |s| parse_params(s, context.clone()), char('>')))(input)?;

        let (input, maybe_symbols) = opt(delimited(
            char('('),
            delimited(ws, |s| parse_symbols(s, context.clone()), ws),
            char(')'),
        ))(input)?;

        let tag = Tag::new(id, maybe_params.unwrap_or_default(), maybe_symbols.unwrap_or_default());

        let mut context = context.borrow_mut();
        context.add_tag(tag.clone());

        Ok((
            input,
            tag,
        ))
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

fn parse_params(input: &str, context: ContextPtr) -> IResult<&str, Vec<TagParam>> {
    let (input, first) = TagParam::parse(input, context.clone())?;

    let (input, mut params) = many0(preceded(comma, |i| TagParam::parse(i, context.clone())))(input)?;

    params.insert(0, first);

    Ok((input, params))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromStr)]
#[display(style = "camelCase")]
pub enum TagId {
    Accol,
    Bar,
    Beam,
    Meter,
    PageFormat,
    Space,
    SystemFormat,
    Staff,
    StemsDown,
    StemsUp,
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

#[derive(Debug, Clone, PartialEq)]
pub enum TagParam {
    Number(f32),
    NumberUnit(f32, Unit),
    String(String),
    VarNumberUnit(String, f32, Unit),
    VarString(String, String),
}

impl TagParam {
    pub fn parse(input: &str, context: ContextPtr) -> IResult<&str, Self> {
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

        let (input, tag) = Tag::parse(input, context).map_err(|e| anyhow!("{}", e))?;

        assert!(input.is_empty());

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
        let tag = parse_tag("\\tie(d e)")?;

        assert_eq!(tag.id, TagId::Tie);
        assert_eq!(tag.symbols.len(), 2);

        assert!(tag.symbols[0].equals(&Note::from_name(Diatonic::D)));
        assert!(tag.symbols[1].equals(&Note::from_name(Diatonic::E)));

        Ok(())
    }

    #[test]
    fn as_number() -> Result<()> {
        let tag = parse_tag("\\staff<1>")?;

        assert_eq!(tag.as_number().unwrap(), 1.0);

        Ok(())
    }
}
