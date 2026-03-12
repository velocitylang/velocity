use std::collections::HashMap;
use std::fs;

use crate::analysis::{check_types};
use crate::grammar::{Expr, Token, TypeEnv};
use crate::lexer::get_next_token;
use crate::parser::Parser;

pub mod analysis;
pub mod grammar;
pub mod lexer;
pub mod parser;

fn main() {
    let source_result = fs::read_to_string("./main.vl");

    if let Ok(source) = source_result {
        println!("{source}");
        let mut tokens: Vec<Token> = Vec::new();

        let mut chars = source.chars().peekable();

        while let Some(next_token) = get_next_token(&mut chars) {
            tokens.push(next_token);
        }

        println!("Tokens: {:?}\n", tokens);

        let mut parser = Parser { tokens, pos: 0 };
        let mut env = TypeEnv {
            idents: HashMap::new()
        };
        let mut ast: Vec<Expr> = Vec::new();

        while parser.peek().is_some() {
            let expr: Expr = parser.parse_statement();
            ast.push(expr);
        }

        println!("AST is: {:?}", ast);

        for expr in ast {
            check_types(expr, &mut env);
        }
    } else {
        println!("Error reading source file");
    }
}
