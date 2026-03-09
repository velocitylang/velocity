use std::fs;
use std::iter::Peekable;
use std::str::Chars;
use crate::grammar::{Expr, Token};
use crate::parser::Parser;

pub mod grammar;
pub mod parser;

fn get_next_token(chars: &mut Peekable<Chars>) -> Option<Token> {
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
        None => None,
        _ => {
            eprintln!("Error: Unknown character {:?}", char.unwrap());
            None
        },
    }
}

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(left, right) => eval(left) + eval(right),
        Expr::Sub(left, right) => eval(left) - eval(right),
        Expr::Mul(left, right) => eval(left) * eval(right),
        Expr::Div(left, right) => eval(left) / eval(right),
    }
}

fn main() {
    let source_result = fs::read_to_string("./main.vl");

    if let Ok(source) = source_result {
        println!("{source}");
        let mut tokens: Vec<Token> = Vec::new();

        let mut chars = source.chars().peekable();

        while let Some(next_token) = get_next_token(&mut chars) {
            tokens.push(next_token);
        }

        println!("Found tokens: {:?}", tokens);

        let mut parser = Parser { tokens, pos: 0 };

        while parser.peek().is_some() {
            let ast: Expr = parser.parse_expr();

            println!("AST is: {:?}", ast);

            let result = eval(&ast);

            println!("Result: {:?}", result);
        }

    } else {
        println!("Error reading source file");
    }
}
