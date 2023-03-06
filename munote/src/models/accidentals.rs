use nom::{
    bytes::complete::is_a,
    combinator::opt,
    error::ErrorKind,
    error_position,
    Err,
    IResult,
};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Accidentals {
    Natural,
    Sharp,
    Flat,
    DoubleSharp,
    DoubleFlat,
}

impl Accidentals {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, accs) = opt(is_a("&#"))(input)?;

        if let Some(accs) = accs {
            let res = match accs {
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
    use anyhow::Result;

    use super::*;

    #[test]
    fn parse_accidentals() -> Result<()> {
        let (_, acc) = Accidentals::parse("")?;
        assert_eq!(acc, Accidentals::Natural);

        let (_, acc) = Accidentals::parse("#")?;
        assert_eq!(acc, Accidentals::Sharp);

        let (_, acc) = Accidentals::parse("&")?;
        assert_eq!(acc, Accidentals::Flat);

        let (_, acc) = Accidentals::parse("##")?;
        assert_eq!(acc, Accidentals::DoubleSharp);

        let (_, acc) = Accidentals::parse("&&")?;
        assert_eq!(acc, Accidentals::DoubleFlat);

        Ok(())
    }
}
