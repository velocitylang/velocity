use std::{collections::HashMap, iter::Peekable};
use std::str::Chars;
use crate::grammar::{Token};

pub fn get_next_token(chars: &mut Peekable<Chars>) -> Option<Token> {
    loop {
        match chars.peek() {
            Some(c) if c.is_whitespace() => {
                chars.next();
            },
            _ => break,
        }
    }

    let char = chars.next();
    match char {
        Some('+') => Some(Token::Plus),
        Some('-') => Some(Token::Minus),
        Some('*') => Some(Token::Star),
        Some('/') => Some(Token::Slash),
        Some('=') => Some(Token::Assign),
        Some('"') => {
            let mut value = String::new();
            while let Some(next_char) = chars.next() {
                if next_char == '"' {
                    return Some(Token::String(value));
                } else {
                    value.push(next_char);
                }
            }

            panic!("Unterminated string literal");
        },
        Some(c) if c.is_digit(10) => {
            let mut num_str = String::from(c);

            while let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            Some(Token::Number(num_str.parse().unwrap()))
        },
        Some(c) if c.is_alphabetic() || c == '_' => {
            let mut s = String::from(c);

            while let Some(&next_char) = chars.peek() {
                if next_char.is_alphabetic() || next_char == '_' {
                    s.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            let mut keywords= HashMap::new();
            keywords.insert("let", Token::Let);
            keywords.insert("make", Token::Make);
            keywords.insert("true", Token::Bool(true));
            keywords.insert("false", Token::Bool(false));

            match keywords.get(s.as_str()) {
                Some(token) => Some(token.clone()),
                None => Some(Token::Ident(s))
            }
        },
        None => None,
        _ => {
            eprintln!("Error: Unknown character {:?}", char.unwrap());
            None
        },
    }
}
