use anyhow::Result;
use regex::Regex;

pub fn inline_comments(input: &str) -> Result<String> {
    let re = Regex::new("%[^\r\n]*")?;
    Ok(re.replace_all(input, "").to_string())
}

pub fn multiline_comments(input: &str) -> Result<String> {
    let re = Regex::new(r"(?s:\(\*.*?\*\))")?;
    Ok(re.replace_all(input, "").to_string())
}

pub fn all_comments(input: &str) -> Result<String> {
    inline_comments(&multiline_comments(input)?)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn remove_inline_comments() -> Result<()> {
        let res = inline_comments(
            "\
a % remove this
b % remove this
",
        )?;

        assert_eq!(&res, "a \nb \n");

        Ok(())
    }

    #[test]
    fn remove_multiline_comments() -> Result<()> {
        let res = multiline_comments(
            "\
a (* remove this
until this *) b(* also this *)
",
        )?;

        assert_eq!(&res, "a  b\n");

        Ok(())
    }
}
