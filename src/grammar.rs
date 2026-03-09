#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
  Number(f64),
  Add(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Token {
  Number(f64),
  Plus,
  Star,
}
