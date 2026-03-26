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
    Block(Vec<Stmt>),
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Call(Box<Expr>, Vec<Expr>),
    Negate(Box<Expr>),
    Number(NumericKind),
    NumberLiteral(String),
    String(String),
    Bool(bool),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    Let(String, Expr, bool, Option<TypeKind>),
    Print(Expr),
    Return(Expr),
    Reassign(String, Expr),
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<FnParam>,
    pub body: Vec<Stmt>,
    pub return_type: Option<TypeKind>,
}
#[derive(Clone, Debug)]
pub struct FnParam {
    pub name: String,
    pub mutable: bool,
    pub modifier: Option<ParamModifier>,
    pub ty: Option<TypeKind>,
}

#[derive(Clone, Debug)]
pub enum ParamModifier {
    Ref,
    Copy,
    Move,
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
    Else,
    Equals,
    Ident(String),
    If,
    LBrace,
    Let,
    LParen,
    Minus,
    Mut,
    Number(NumericKind),
    NumberLiteral(String),
    Plus,
    Print,
    RBrace,
    Return,
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
    Unit,
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
