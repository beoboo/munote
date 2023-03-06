use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, verify},
    IResult,
    number::complete::float,
    sequence::{delimited, terminated, Tuple},
};
use nom::sequence::tuple;

use crate::{
    models::{string, ws},
    unit::Unit,
};
use crate::models::Span;

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
    pub fn parse(input: Span) -> IResult<Span, Self> {
        terminated(
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
                map(|s| parse_string(s), |s| TagParam::String(s.to_string())),
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

fn parse_string(input: Span) -> IResult<Span, Span> {
    let not_quote = is_not("\"");

    let check_string = verify(not_quote, |s: &Span| !s.is_empty());

    let (input, s) = delimited(char('"'), check_string, char('"'))(input)?;

    Ok((input, s))
}

fn parse_number_unit(input: Span) -> IResult<Span, (f32, Unit)> {
    let (input, number) = float(input)?;
    let (input, unit) = Unit::parse(input)?;

    Ok((input, (number, unit)))
}

fn parse_var_string(input: Span) -> IResult<Span, (String, String)> {
    let (input, (name, _, val)) =
        tuple((
            string,
            delimited(ws, char('='), ws),
            parse_string
        ))(input)?;

    Ok((input, (name.to_string(), val.to_string())))
}

fn parse_var_number(input: Span) -> IResult<Span, (String, f32)> {
    let (input, (name, _, num)) =
        tuple((
            string,
            delimited(ws, char('='), ws),
            float,
        ))(input)?;

    Ok((input, (name.to_string(), num)))
}

fn parse_var_number_unit(input: Span) -> IResult<Span, (String, f32, Unit)> {
    let (input, (name, _, (num, unit))) =
        (string, delimited(ws, char('='), ws), |s| {
            parse_number_unit(s)
        })
            .parse(input)?;

    Ok((input, (name.to_string(), num, unit)))
}


#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};
    use strum::IntoEnumIterator;

    use crate::unit::Unit;

    use super::*;

    fn parse_tag_param(input: &str) -> Result<TagParam> {
        let (input, param) =
            TagParam::parse(Span::new(input)).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(param)
    }

    #[test]
    fn parse_string_param() -> Result<()> {
        assert_eq!(parse_tag_param("\"2/4\"")?, TagParam::String("2/4".into()));
        assert_eq!(parse_tag_param("\"F#m7\"")?, TagParam::String("F#m7".into()));
        assert_eq!(parse_tag_param("\"\\166\"")?, TagParam::String("\\166".into()));

        Ok(())
    }

    #[test]
    fn parse_number_param() -> Result<()> {
        assert_eq!(parse_tag_param("1")?, TagParam::Number(1.0));

        assert_eq!(
            parse_tag_param("-0.5")?,
            TagParam::Number(-0.5)
        );

        Ok(())
    }

    #[test]
    fn parse_variable_number_unit_param() -> Result<()> {
        assert_eq!(
            parse_tag_param("dx=1cm")?,
            TagParam::VarNumberUnit("dx".into(), 1.0, Unit::Cm)
        );

        Ok(())
    }

    #[test]
    fn parse_number_units() -> Result<()> {
        for u in Unit::iter() {
            assert_eq!(
                parse_tag_param(&format!("1{u}"))?,
                TagParam::NumberUnit(1.0, u)
            );
        }

        Ok(())
    }

    #[test]
    fn parse_variable_string_param() -> Result<()> {
        assert_eq!(
            parse_tag_param("type=\"thinBrace\"")?,
            TagParam::VarString("type".into(), "thinBrace".into())
        );

        Ok(())
    }
}
