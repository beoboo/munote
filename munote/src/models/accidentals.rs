use nom::{
    bytes::complete::is_a,
    combinator::opt,
    error::ErrorKind,
    error_position,
    Err,
    IResult,
};
use serde::Deserialize;
use crate::models::Span;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Accidentals {
    Natural,
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}

impl Accidentals {
    pub fn parse(input: Span) -> IResult<Span, Self> {
        let (input, accs) = opt(is_a("&#"))(input)?;

        if let Some(accs) = accs {
            let res = match *accs.fragment() {
                "#" => Accidentals::Sharp,
                "&" => Accidentals::Flat,
                "##" => Accidentals::DoubleSharp,
                "&&" => Accidentals::DoubleFlat,
                _ => {
                    return Err(Err::Error(error_position!(
                        input,
                        ErrorKind::IsNot
                    )))
                },
            };
            Ok((input, res))
        } else {
            Ok((input, Accidentals::Natural))
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Result};

    use super::*;

    fn parse_accidental(input: &str) -> Result<Accidentals> {
        let (input, parsed) =
            Accidentals::parse(Span::new(input)).map_err(|e| anyhow!("{}", e))?;

        assert_eq!(*input.fragment(), "");

        Ok(parsed)
    }

    #[test]
    fn parse_accidentals() -> Result<()> {
        assert_eq!(parse_accidental("")?, Accidentals::Natural);
        assert_eq!(parse_accidental("#")?, Accidentals::Sharp);
        assert_eq!(parse_accidental("&")?, Accidentals::Flat);
        assert_eq!(parse_accidental("##")?, Accidentals::DoubleSharp);
        assert_eq!(parse_accidental("&&")?, Accidentals::DoubleFlat);

        Ok(())
    }
}
