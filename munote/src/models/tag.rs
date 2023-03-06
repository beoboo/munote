use std::{any::Any, str::FromStr};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char},
    combinator::{map, map_res, opt, value, verify},
    multi::many0,
    number::complete::float,
    sequence::{delimited, preceded, Tuple},
    IResult,
};
use parse_display::{Display, FromStr};
use strum::EnumIter;

use lazy_static::lazy_static;

use crate::{
    context::ContextPtr,
    models::{comma, ws},
    symbol::{parse_symbols, Symbol},
};

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub params: Vec<TagParam>,
    pub symbols: Vec<Box<dyn Symbol>>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && same_symbols(self, other)
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
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }

    fn clone_box(&self) -> Box<dyn Symbol> {
        Box::new((*self).clone())
    }
}

impl Tag {
    pub fn new(
        id: TagId,
        params: Vec<TagParam>,
        symbols: Vec<Box<dyn Symbol>>,
    ) -> Self {
        Self {
            id,
            params,
            symbols,
        }
    }

    pub fn from_id(id: TagId) -> Self {
        Self::new(id, Vec::new(), Vec::new())
    }

    pub fn parse(input: &str, mut context: ContextPtr) -> IResult<&str, Self> {
        let (input, id) = map_res(
            alt((preceded(char('\\'), alpha1), tag("|"))),
            TagId::lookup,
        )(input)?;

        let (input, maybe_params) =
            opt(delimited(char('<'), |s| parse_params(s), char('>')))(input)?;

        let (input, maybe_symbols) = opt(delimited(
            char('('),
            delimited(ws, |s| parse_symbols(s, context.clone()), ws),
            char(')'),
        ))(input)?;

        let tag = Tag::new(
            id,
            maybe_params.unwrap_or_default(),
            maybe_symbols.unwrap_or_default(),
        );

        let mut context = context.borrow_mut();
        context.add_tag(tag.clone());

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

fn parse_params(input: &str) -> IResult<&str, Vec<TagParam>> {
    let (input, first) = TagParam::parse(input)?;

    let (input, mut params) =
        many0(preceded(comma, |i| TagParam::parse(i)))(input)?;

    params.insert(0, first);

    Ok((input, params))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromStr)]
#[display(style = "camelCase")]
pub enum TagId {
    Accidental,
    Accol,
    Alter,
    Bar,
    Beam,
    Clef,
    Composer,
    DoubleBar,
    Footer,
    Harmony,
    Key,
    Meter,
    PageFormat,
    Set,
    Space,
    SystemFormat,
    Staff,
    StemsDown,
    StemsUp,
    Tie,
    Title,
    Tuplet,
}

impl TagId {
    fn lookup(name: &str) -> Result<TagId> {
        lazy_static! {
            static ref TAG_ID_LOOKUP: std::collections::HashMap<&'static str, TagId> = {
                let mut m = std::collections::HashMap::new();
                m.insert("acc", TagId::Accidental);
                m.insert("|", TagId::Bar);
                m
            };
        }
        if TAG_ID_LOOKUP.contains_key(name) {
            Ok(TAG_ID_LOOKUP[name])
        } else {
            TagId::from_str(name).map_err(|e| anyhow!("{e}"))
        }
    }
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
    VarNumber(String, f32),
    VarNumberUnit(String, f32, Unit),
    VarString(String, String),
}

impl TagParam {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(
            ws,
            alt((
                map(
                    |s| parse_var_string(s),
                    |(v, s)| TagParam::VarString(v, s),
                ),
                map(
                    |s| parse_var_number_unit(s),
                    |(v, n, u)| TagParam::VarNumberUnit(v, n, u),
                ),
                map(
                    |s| parse_var_number(s),
                    |(v, n)| TagParam::VarNumber(v, n),
                ),
                map(|s| parse_string(s), TagParam::String),
                map(
                    |s| parse_number_unit(s),
                    |(n, u)| TagParam::NumberUnit(n, u),
                ),
                map(|s| float(s), TagParam::Number),
            )),
            ws,
        )(input)
    }
}

fn parse_string(input: &str) -> IResult<&str, String> {
    let not_quote_slash = is_not("\"\\");

    let check_string = verify(not_quote_slash, |s: &str| !s.is_empty());

    let (input, s) = delimited(char('"'), check_string, char('"'))(input)?;

    Ok((input, s.to_string()))
}

fn parse_unit(input: &str) -> IResult<&str, Unit> {
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

fn parse_number_unit(input: &str) -> IResult<&str, (f32, Unit)> {
    let (input, number) = float(input)?;
    let (input, unit) = parse_unit(input)?;

    Ok((input, (number, unit)))
}

fn parse_var_string(input: &str) -> IResult<&str, (String, String)> {
    let (input, (name, _, val)) =
        (alpha1, delimited(ws, char('='), ws), |s| parse_string(s))
            .parse(input)?;

    Ok((input, (name.to_string(), val)))
}

fn parse_var_number_unit(input: &str) -> IResult<&str, (String, f32, Unit)> {
    let (input, (name, _, (num, unit))) =
        (alpha1, delimited(ws, char('='), ws), |s| {
            parse_number_unit(s)
        })
            .parse(input)?;

    Ok((input, (name.to_string(), num, unit)))
}

fn parse_var_number(input: &str) -> IResult<&str, (String, f32)> {
    let (input, (name, _, num)) =
        (alpha1, delimited(ws, char('='), ws), |s| float(s)).parse(input)?;

    Ok((input, (name.to_string(), num)))
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use strum::IntoEnumIterator;

    use crate::{
        accidentals::Accidentals,
        note::{Diatonic, Note},
    };

    use super::*;

    fn parse_tag(input: &str) -> Result<Tag> {
        let context = ContextPtr::default();

        let (input, tag) =
            Tag::parse(input, context).map_err(|e| anyhow!("{}", e))?;

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

        assert_eq!(tag.id, TagId::Accol);
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

    #[test]
    fn compound() -> Result<()> {
        let tag = parse_tag("\\accidental<size=1.4>(d&)")?;

        assert_eq!(tag.id, TagId::Accidental);
        assert_eq!(tag.params, vec![TagParam::VarNumber("size".into(), 1.4)]);
        assert!(tag.symbols[0].equals(
            &Note::from_name(Diatonic::D).with_accidentals(Accidentals::Flat)
        ));

        let tag = parse_tag("\\tuplet<\"-3-\",dy1=-3, dy2=1>(c/6 d e&)")?;

        assert_eq!(tag.id, TagId::Tuplet);

        Ok(())
    }

    #[test]
    fn eat_whitespaces() -> Result<()> {
        let tag = parse_tag("\\systemFormat< dx=1cm>")?;

        assert_eq!(
            tag.params,
            vec![TagParam::VarNumberUnit("dx".into(), 1.0, Unit::Cm)]
        );

        Ok(())
    }

    #[test]
    fn lookup_variants() -> Result<()> {
        assert_eq!(parse_tag("\\acc")?.id, TagId::Accidental);
        assert_eq!(parse_tag("|")?.id, TagId::Bar);

        Ok(())
    }
}
