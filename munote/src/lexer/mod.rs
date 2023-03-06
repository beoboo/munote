use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, bail};
use anyhow::Result;
use crate::accidentals::Accidentals;
use crate::dots::Dots;
use crate::tag_id::TagId;

type PeekableChar<'a> = Peekable<Chars<'a>>;

#[derive(Default)]
pub struct Lexer {}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Accidental(Accidentals),
    Dots(Dots),
    Identifier(String),
    Number(f32),
    Parens(char),
    TagId(TagId),
    String(String),
}

impl Lexer {
    pub fn lex(&self, input: &str) -> Result<Vec<Token>> {
        let mut it = input.chars().peekable();
        let mut tokens = Vec::new();

        loop {
            let ch = peek(&mut it);
            let t = match ch {
                // Skips whitespaces
                ' ' | '\t' | '\n' => {
                    advance(&mut it);
                    continue;
                }
                '+' | '-' | '0'..='9' => number(&mut it)?,
                '{' | '[' | '(' | ')' | ']' | '}' => parens(&mut it),
                '"' => string(&mut it)?,
                '.' => dots(&mut it)?,
                '#' | '&' => accidentals(&mut it)?,
                '\\' => tag_id(&mut it)?,
                '\0' => break,
                ch => {
                    if ch.is_alphabetic() {
                        identifier(&mut it)?
                    } else {
                        bail!("Invalid input: {}", ch);
                    }
                }
            };

            tokens.push(t);
        }

        Ok(tokens)
    }
}

fn number(it: &mut PeekableChar) -> Result<Token> {
    let ch = peek(it);

    let sign = match ch {
        '+' | '-' => {
            if !is_digit(peek_next(it)) {
                return Err(anyhow!("Unexpected {ch}"));
            }

            advance(it);
            ch
        }
        _ => '+'
    };

    let mut number = String::new();

    while is_digit(peek(it)) {
        number.push(advance(it));
    }

    if it.peek() == Some(&'.') {
        number.push(advance(it)); // Consume '.'

        while is_digit(peek(it)) {
            number.push(advance(it));
        }
    }

    let mut number = number.parse::<f32>().unwrap();
    if sign == '-' {
        number = -number
    }

    Ok(Token::Number(number))
}

fn dots(it: &mut PeekableChar) -> Result<Token> {
    let mut count = 0;
    while peek(it) == '.' {
        advance(it);
        count += 1;
    }

    let dots = match count {
        1 => Dots::Single,
        2 => Dots::Double,
        3 => Dots::Triple,
        _ => bail!("Invalid number of dots: {count}",)
    };

    Ok(Token::Dots(dots))
}

fn accidentals(it: &mut PeekableChar) -> Result<Token> {
    let mut s = String::new();
    while is_accidental(peek(it)) {
        s.push(advance(it));
    }

    let acc = match s.as_str() {
        "&" => Accidentals::Flat,
        "#" => Accidentals::Sharp,
        "&&" => Accidentals::DoubleFlat,
        "##" => Accidentals::DoubleSharp,
        _ => bail!("Invalid accidentals: {s}",)
    };

    Ok(Token::Accidental(acc))
}

fn identifier(it: &mut PeekableChar) -> Result<Token> {
    let mut s = String::from(advance(it));

    while is_alphanum(peek(it)) {
        s.push(advance(it));
    }

    Ok(Token::Identifier(s))
}

fn parens(it: &mut PeekableChar) -> Token {
    Token::Parens(advance(it))
}

fn string(it: &mut PeekableChar) -> Result<Token> {
    let mut string = String::new();

    // Consume '"'.
    advance(it);

    while peek(it) != '"' && !is_at_end(it) {
        string.push(advance(it));
    }

    if is_at_end(it) {
        bail!("Unterminated string.");
    }

    // Consume '"'.
    advance(it);

    Ok(Token::String(string))
}


fn tag_id(it: &mut PeekableChar) -> Result<Token> {
    advance(it); // Removes '\'

    let ch = peek_next(it);

    if !(is_alpha(ch)) {
        return Err(anyhow!("Unexpected {ch}"));
    }

    let mut id = String::new();

    while is_alpha(peek(it)) {
        id.push(advance(it));
    }

    Ok(Token::TagId(TagId::lookup(&id)?))
}

fn peek(it: &mut PeekableChar) -> char {
    match it.peek() {
        Some(t) => *t,
        None => '\0'
    }
}

fn peek_next(it: &mut PeekableChar) -> char {
    let mut next_it = it.clone();
    next_it.next();

    match next_it.peek() {
        Some(t) => *t,
        None => '\0'
    }
}

fn is_at_end(it: &mut Peekable<Chars>) -> bool {
    peek(it) == '\0'
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

fn is_alpha(ch: char) -> bool {
    ch.is_alphanumeric()
}

fn is_alphanum(ch: char) -> bool {
    is_alpha(ch) | is_digit(ch)
}

fn is_accidental(ch: char) -> bool {
    ch == '&' || ch == '#'
}

fn advance(it: &mut PeekableChar) -> char {
    match it.next() {
        Some(t) => t,
        None => '\0'
    }
}

#[cfg(test)]
mod tests {
    use crate::accidentals::Accidentals;
    use crate::dots::Dots;
    use crate::tag_id::TagId::Accidental;
    use super::*;

    fn lex(input: &str) -> Result<Vec<Token>> {
        let lexer = Lexer::default();

        lexer.lex(input)
    }

    #[test]
    fn lex_identifier() -> Result<()> {
        assert_eq!(lex("a")?, vec![Token::Identifier("a".into())]);
        assert_eq!(lex("abc")?, vec![Token::Identifier("abc".into())]);
        assert_eq!(lex("a1")?, vec![Token::Identifier("a1".into())]);

        Ok(())
    }

    #[test]
    fn lex_number() -> Result<()> {
        assert_eq!(lex("1")?, vec![Token::Number(1.0)]);
        assert_eq!(lex("2.0")?, vec![Token::Number(2.0)]);
        assert_eq!(lex("+3.0")?, vec![Token::Number(3.0)]);
        assert_eq!(lex("-4.0")?, vec![Token::Number(-4.0)]);

        Ok(())
    }

    #[test]
    fn lex_parens() -> Result<()> {
        assert_eq!(lex("{[()]}")?, vec![
            Token::Parens('{'),
            Token::Parens('['),
            Token::Parens('('),
            Token::Parens(')'),
            Token::Parens(']'),
            Token::Parens('}'),
        ]);

        Ok(())
    }

    #[test]
    fn lex_tag_id() -> Result<()> {
        assert_eq!(lex("\\tie")?, vec![Token::TagId(TagId::Tie)]);

        Ok(())
    }

    #[test]
    fn lex_string() -> Result<()> {
        assert_eq!(lex("\"hello\"")?, vec![Token::String("hello".into())]);

        Ok(())
    }

    #[test]
    fn lex_accidentals() -> Result<()> {
        assert_eq!(lex("& # && ##")?, vec![
            Token::Accidental(Accidentals::Flat),
            Token::Accidental(Accidentals::Sharp),
            Token::Accidental(Accidentals::DoubleFlat),
            Token::Accidental(Accidentals::DoubleSharp),
        ]);

        Ok(())
    }

    #[test]
    fn lex_dots() -> Result<()> {
        assert_eq!(lex(". .. ...")?, vec![
            Token::Dots(Dots::Single),
            Token::Dots(Dots::Double),
            Token::Dots(Dots::Triple),
        ]);

        Ok(())
    }
}