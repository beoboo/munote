use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, verify},
    number::complete::float,
    sequence::{delimited, terminated, Tuple},
    IResult,
};

use crate::{
    models::{string, ws},
    unit::Unit,
};

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

fn parse_number_unit(input: &str) -> IResult<&str, (f32, Unit)> {
    let (input, number) = float(input)?;
    let (input, unit) = Unit::parse(input)?;

    Ok((input, (number, unit)))
}

fn parse_var_string(input: &str) -> IResult<&str, (String, String)> {
    let (input, (name, _, val)) =
        (string, delimited(ws, char('='), ws), |s| parse_string(s))
            .parse(input)?;

    Ok((input, (name.to_string(), val)))
}

fn parse_var_number(input: &str) -> IResult<&str, (String, f32)> {
    let (input, (name, _, num)) =
        (string, delimited(ws, char('='), ws), |s| float(s)).parse(input)?;

    Ok((input, (name.to_string(), num)))
}

fn parse_var_number_unit(input: &str) -> IResult<&str, (String, f32, Unit)> {
    let (input, (name, _, (num, unit))) =
        (string, delimited(ws, char('='), ws), |s| {
            parse_number_unit(s)
        })
            .parse(input)?;

    Ok((input, (name.to_string(), num, unit)))
}
