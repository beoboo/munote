use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};
use parse_display::Display;
use strum::EnumIter;
use crate::models::Span;

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

impl Unit {
    pub fn parse(input: Span) -> IResult<Span, Self> {
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
}
