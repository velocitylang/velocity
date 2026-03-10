#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
  Number(f64),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Var(String),
  Assign(String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Ident(String),
  Let,
  Make,
  Assign,
  Number(f64),
  Equals,
  Plus,
  Minus,
  Star,
  Slash,
}
