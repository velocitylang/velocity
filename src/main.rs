use std::collections::HashMap;
use std::fs;

use crate::grammar::{Expr, Token, Value};
use crate::lexer::get_next_token;
use crate::parser::Parser;

pub mod grammar;
pub mod lexer;
pub mod parser;

pub struct Binding {
    pub value: Value,
    pub mutable: bool,
}

struct Env { 
    idents: HashMap<String, Binding>,
}

fn eval(expr: &Expr, env: &mut Env) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::String(s) => Value::String(s.clone()),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Add(left, right) => {
            let l = eval(left, env);
            let r = eval(right, env);

            match (l, r) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                _ => panic!("Can only add numbers"),
            }
        },
        Expr::Sub(left, right) => {
            let l = eval(left, env);
            let r = eval(right, env);

            match(l, r) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                _ => panic!("Can only subtract numbers"),
            }
        },
        Expr::Mul(left, right) => {
            let l = eval(left, env);
            let r = eval(right, env);

            match (l, r) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                _ => panic!("Can only multiply numbers")
            }
        },
        Expr::Div(left, right) => {
            let l = eval(left, env);
            let r = eval(right, env);

            match (l, r) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                _ => panic!("Can only divide numbers")
            }
        },
        Expr::Var(ident) => {
            let value = env.idents.get(ident);
            match value {
                Some(b) => b.value.clone(),
                _ => panic!("Identifier {ident} not found")
            }
        },
        Expr::LetDecl(ident, expr) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let value = eval(expr, env);
            env.idents.insert(String::from(ident), Binding { value: value.clone(), mutable: true });
            value
        },
        Expr::MakeDecl(ident, expr) => {
            if let Some(_) = env.idents.get(ident) {
                panic!("Cannot redeclare existing identifier {ident}");
            }

            let value = eval(expr, env);
            env.idents.insert(String::from(ident), Binding { value: value.clone(), mutable: false });
            value
        },
        Expr::Reassign(ident, expr) => {
            let value = eval(expr, env);

            match env.idents.get_mut(ident) {
                Some(b) => {
                    if !b.mutable {
                        panic!("Cannot reassign immutable identifier {ident}")
                    }
                    b.value = value.clone();
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
