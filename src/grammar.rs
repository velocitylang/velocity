#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
  Number(f64),
  String(String),
  Bool(bool),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Var(String),
  LetDecl(String, Box<Expr>),
  MakeDecl(String, Box<Expr>),
  Reassign(String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  String(String),
  Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Ident(String),
  Let,
  Make,
  Assign,
  Number(f64),
  String(String),
  Bool(bool),
  Equals,
  Plus,
  Minus,
  Star,
  Slash,
}
