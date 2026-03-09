use crate::grammar::{Expr, Token};

pub struct Parser {
  pub tokens: Vec<Token>,
  pub pos: usize,
}

impl Parser {
  fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.pos)
  }

  fn consume(&mut self) -> Option<&Token> {
    let token = self.tokens.get(self.pos);
    if token.is_some() {
      self.pos += 1;
    }
    token
  }

  pub fn parse_expr(&mut self) -> Expr {
    let mut left = self.parse_term();

    while matches!(self.peek(), Some(&Token::Plus)) {
      self.consume();
      let right = self.parse_term();
      left = Expr::Add(Box::new(left), Box::new(right));
    }
    left
  }

  fn parse_term(&mut self) -> Expr {
    let mut left = self.parse_number();

    while matches!(self.peek(), Some(Token::Star)) {
      self.consume();
      let right = self.parse_number();
      left = Expr::Mul(Box::new(left), Box::new(right));
    }
    left
  }

  fn parse_number(&mut self) -> Expr {
    match self.consume() {
      Some(Token::Number(n)) => Expr::Number(*n),
      _ => panic!("Expected a number, but fond somethine else!"),
    }
  }
}