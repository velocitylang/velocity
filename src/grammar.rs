use std::collections::HashMap;

pub struct TypeEnv { 
    pub idents: HashMap<String, TypeBinding>,
}

pub struct TypeBinding {
    pub ty: TypeKind,
    pub mutable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(NumericKind),
    NumberLiteral(String),
    String(String),
    Bool(bool),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    LetDecl(String, Box<Expr>, Option<TypeKind>),
    MakeDecl(String, Box<Expr>, Option<TypeKind>),
    Print(Box<Expr>),
    Reassign(String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(NumericKind),
    NumberLiteral(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Assign,
    Bool(bool),
    Colon,
    Equals,
    Ident(String),
    Let,
    LParen,
    Make,
    Minus,
    Number(NumericKind),
    NumberLiteral(String),
    Plus,
    Print,
    RParen,
    Slash,
    Star,
    String(String),
    Type(TypeKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    String,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NumericKind {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}
