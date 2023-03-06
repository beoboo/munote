use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, bail};
use anyhow::Result;

type PeekableChar<'a> = Peekable<Chars<'a>>;

#[derive(Default)]
pub struct Lexer {}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f32),
    Parens(char),
}

impl Lexer {
    pub fn lex(&self, input: &str) -> Result<Vec<Token>> {
        let mut it = input.chars().peekable();
        let mut tokens = Vec::new();

        loop {
            let ch = peek(&mut it);
            let t = match ch {
                '+' | '-' | '0'..='9' => number(&mut it)?,
                '{' | '[' | '(' | ')' | ']' | '}' => parens(&mut it),
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

fn identifier(it: &mut PeekableChar) -> Result<Token> {
    let mut s = String::from(advance(it));

    while peek(it).is_alphanumeric() {
        s.push(advance(it));
    }

    Ok(Token::Identifier(s))
}

fn parens(it: &mut PeekableChar) -> Token {
    Token::Parens(advance(it))
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

fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}


fn advance(it: &mut PeekableChar) -> char {
    match it.next() {
        Some(t) => t,
        None => '\0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Result<Vec<Token>> {
        let lexer = Lexer::default();

        lexer.lex(input)
    }

    #[test]
    fn lex_string() -> Result<()> {
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
}