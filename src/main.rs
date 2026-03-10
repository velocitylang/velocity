use std::collections::HashMap;
use std::fs;
use std::iter::Peekable;
use std::str::Chars;
use crate::grammar::{Expr, Token};
use crate::parser::Parser;

pub mod grammar;
pub mod parser;

pub struct Binding {
    pub value: f64,
    pub mutable: bool,
}

struct Env { 
    idents: HashMap<String, Binding>,
}

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
        Some('=') => Some(Token::Assign),
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
        Some(c) if c.is_alphabetic() => {
            let mut s = String::from(c);

            while let Some(&next_char) = chars.peek() {
                if next_char.is_alphabetic() {
                    s.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            let mut keywords= HashMap::new();
            keywords.insert("let", Token::Let);
            keywords.insert("make", Token::Make);

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

fn eval(expr: &Expr, env: &mut Env) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(left, right) => eval(left, env) + eval(right, env),
        Expr::Sub(left, right) => eval(left, env) - eval(right, env),
        Expr::Mul(left, right) => eval(left, env) * eval(right, env),
        Expr::Div(left, right) => eval(left, env) / eval(right, env),
        Expr::Var(ident) => {
            let value = env.idents.get(ident);
            match value {
                Some(b) => return b.value,
                _ => panic!("Identifier {ident} not found")
            }
        },
        Expr::LetDecl(ident, expr) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let value = eval(expr, env);
            env.idents.insert(String::from(ident), Binding { value, mutable: true });
            value
        },
        Expr::MakeDecl(ident, expr) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let value = eval(expr, env);
            env.idents.insert(String::from(ident), Binding { value, mutable: false });
            value
        },
        Expr::Reassign(ident, expr) => {
            let value = eval(expr, env);

            match env.idents.get_mut(ident) {
                Some(b) => {
                    if !b.mutable {
                        panic!("Cannot reassign immutable identifier {ident}")
                    }
                    b.value = value;
                    value
                },
                None => panic!("Cannot reassign undefined identifier {ident}")
            }
        },
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
        let mut env = Env {
            idents: HashMap::new()
        };

        while parser.peek().is_some() {
            let ast: Expr = parser.parse_assignment();

            println!("AST is: {:?}", ast);

            let result = eval(&ast, &mut env);

            println!("Result: {:?}", result);
        }

    } else {
        println!("Error reading source file");
    }
}
