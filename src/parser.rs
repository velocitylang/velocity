use crate::grammar::{Expr, Token, TypeKind};

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

    pub fn parse_statement(&mut self) -> Expr {
        let token = self.peek();

        match token {
            Some(Token::Print) => self.parse_print(),
            _ => self.parse_assignment(),
        }
    }

    fn parse_assignment(&mut self) -> Expr {
        if matches!(self.peek(), Some(Token::Let)) {
            self.consume();

            let mut mutable = false;

            if matches!(self.peek(), Some(Token::Mut)) {
                self.consume();
                mutable = true;
            }

            if let Some(Token::Ident(name)) = self.consume() {
                let name = name.clone();
                let mut ty: Option<TypeKind> = None;

                if matches!(self.peek(), Some(Token::Colon)) {
                    self.consume();
                    ty = match self.consume() {
                        Some(Token::Type(t)) => Some(t.clone()),
                        _ => None,
                    }
                }

                if matches!(self.peek(), Some(Token::Assign)) {
                    self.consume();
                    let right = self.parse_assignment();
                    return Expr::LetDecl(name, Box::new(right), mutable, ty);
                }
            }
        }

        if let Some(Token::Ident(name)) = self.peek() {
            let name = name.clone();

            if matches!(self.tokens.get(self.pos + 1), Some(Token::Assign)) {
                self.consume(); // ident
                self.consume(); // =
                let right = self.parse_assignment();
                return Expr::Reassign(name, Box::new(right));
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
        let token = self.consume();
        match token {
            Some(Token::Minus) => {
                let expr = self.parse_primary();
                Expr::Negate(Box::new(expr))
            }
            Some(Token::NumberLiteral(n)) => Expr::NumberLiteral(n.clone()),
            Some(Token::String(s)) => Expr::String(s.clone()),
            Some(Token::Bool(b)) => Expr::Bool(*b),
            Some(Token::Ident(name)) => Expr::Var(name.clone()),
            _ => panic!("Expected a number, but found {:?}", token),
        }
    }

    fn parse_print(&mut self) -> Expr {
        self.consume(); // print
        self.consume(); // (
        let expr = self.parse_expr();
        self.consume(); // )
        Expr::Call(Box::new(Expr::Var(String::from("print"))), vec![expr])
    }
}
