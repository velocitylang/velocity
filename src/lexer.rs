use std::{collections::HashMap, iter::Peekable};
use std::str::Chars;
use crate::grammar::{Token, TypeKind};

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
        Some('(') => Some(Token::LParen),
        Some(')') => Some(Token::RParen),
        Some('{') => Some(Token::LBrace),
        Some('}') => Some(Token::RBrace),
        Some(':') => Some(Token::Colon),
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
            let mut has_dot = false;

            while let Some(&next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    num_str.push(chars.next().unwrap());
                } else if next_char == '.' && !has_dot {
                    has_dot = true;
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            Some(Token::NumberLiteral(num_str))
        }
        Some(c) if c.is_alphanumeric() || c == '_' => {
            let mut s = String::from(c);

            while let Some(&next_char) = chars.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    s.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            let mut tokens= HashMap::new();
            tokens.insert("let", Token::Let);
            tokens.insert("mut", Token::Mut);
            tokens.insert("true", Token::Bool(true));
            tokens.insert("false", Token::Bool(false));
            tokens.insert("print", Token::Print);
            tokens.insert("if", Token::If);
            tokens.insert("else", Token::Else);
            tokens.insert("return", Token::Return);
            tokens.insert("string", Token::Type(TypeKind::String));
            tokens.insert("bool", Token::Type(TypeKind::Bool));
            tokens.insert("i8", Token::Type(TypeKind::I8));
            tokens.insert("i16", Token::Type(TypeKind::I16));
            tokens.insert("i32", Token::Type(TypeKind::I32));
            tokens.insert("i64", Token::Type(TypeKind::I64));
            tokens.insert("u8", Token::Type(TypeKind::U8));
            tokens.insert("u16", Token::Type(TypeKind::U16));
            tokens.insert("u32", Token::Type(TypeKind::U32));
            tokens.insert("u64", Token::Type(TypeKind::U64));
            tokens.insert("f32", Token::Type(TypeKind::F32));
            tokens.insert("f64", Token::Type(TypeKind::F64));

            match tokens.get(s.as_str()) {
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
