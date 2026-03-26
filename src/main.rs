use std::collections::HashMap;
use std::fs;

use crate::analysis::{check_stmt_types};
use crate::grammar::{Stmt, Token, TypeEnv};
use crate::lexer::get_next_token;
use crate::parser::Parser;
use crate::vir::{Item, Program, lower_program};

pub mod analysis;
pub mod grammar;
pub mod lexer;
pub mod parser;
pub mod ppv;
pub mod vir;

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
        let mut ast: Program = Program { items: Vec::new() };

        while parser.peek().is_some() {
            let stmt: Stmt = parser.parse_stmt();
            ast.items.push(Item::Stmt(stmt));
        }

        println!("AST is: {:?}\n", ast);

        for item in &mut ast.items {
            match item {
                Item::Stmt(stmt) => {
                    check_stmt_types(stmt, &mut env);
                },
                Item::Function(_fnct) => {
                    // check function types
                }
            }
        }

        let _vir = lower_program(&ast);
    } else {
        println!("Error reading source file");
    }
}
