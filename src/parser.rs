use crate::grammar::{Expr, Token};

pub struct Parser {
  pub tokens: Vec<Token>,
  pub pos: usize,
}

impl Parser {
  pub fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.pos)
  }

  fn consume(&mut self) -> Option<&Token> {
    let token = self.tokens.get(self.pos);
    if token.is_some() {
      self.pos += 1;
    }
    token
  }

  pub fn parse_assignment(&mut self) -> Expr {
    if matches!(self.peek(), Some(Token::Let) | Some(Token::Make)) {
      self.consume();

      if let Some(Token::Ident(name)) = self.consume() {
        let name = name.clone();

        if matches!(self.peek(), Some(Token::Assign)) {
          self.consume();
          let right = self.parse_assignment();
          return Expr::Assign(name, Box::new(right));
        }
      }
    }

    self.parse_expr()
  }

  fn parse_expr(&mut self) -> Expr {
    let mut left = self.parse_term();

    while let Some(token) = self.peek() {
      match token {
        Token::Plus => {
          self.consume();
          let right = self.parse_term();
          left = Expr::Add(Box::new(left), Box::new(right));
        },
        Token::Minus => {
          self.consume();
          let right = self.parse_term();
          left = Expr::Sub(Box::new(left), Box::new(right));
        },
        _ => break,
      }
    }
    left
  }

  fn parse_term(&mut self) -> Expr {
    let mut left = self.parse_primary();

    while let Some(token) = self.peek() {
      match token {
        Token::Star => {
          self.consume();
          let right = self.parse_primary();
          left = Expr::Mul(Box::new(left), Box::new(right));
        },
        Token::Slash => {
          self.consume();
          let right = self.parse_primary();
          left = Expr::Div(Box::new(left), Box::new(right));
        },
        _ => break,
      }
      
    }
    left
  }

  fn parse_primary(&mut self) -> Expr {
    match self.consume() {
      Some(Token::Number(n)) => Expr::Number(*n),
      _ => panic!("Expected a number, but found something else!"),
    }
  }
}