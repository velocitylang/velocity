use crate::{grammar::{Expr, Stmt, Token, TypeKind}};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let token = self.peek();

        match token {
            Some(Token::Print) => self.parse_print(),
            Some(Token::Let) => self.parse_let_stmt(),
            Some(Token::Ident(_)) => {
                if let Some(Token::Assign) = self.peek_ahead(1) {
                    self.parse_assign_stmt()
                } else {
                    Stmt::ExprStmt(self.parse_expr())
                }
            }
            Some(Token::Assign) => self.parse_assign_stmt(),
            Some(Token::Return) => Stmt::Return(self.parse_expr()),
            Some(Token::If) => Stmt::ExprStmt(self.parse_if_expr()),
            _ => Stmt::ExprStmt(self.parse_expr()),
        }
    }

    fn parse_let_stmt(&mut self) -> Stmt {
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
                let right = if matches!(self.peek(), Some(Token::If)) {
                    self.parse_if_expr()
                } else {
                    self.parse_expr()
                };

                Stmt::Let(name, right, mutable, ty)
            } else {
                panic!("Expected assign (=) in let statement");
            }
        } else {
            panic!("Expected identifier in let statement");
        }
    }

    fn parse_assign_stmt(&mut self) -> Stmt {
        if let Some(Token::Ident(name)) = self.peek() {
            let name = name.clone();

            if matches!(self.tokens.get(self.pos + 1), Some(Token::Assign)) {
                self.consume(); // ident
                self.consume(); // =
                let right = self.parse_expr();
                return Stmt::Reassign(name, right);
            }
        }

        Stmt::ExprStmt(self.parse_expr())
    }
    
    fn parse_if_expr(&mut self) -> Expr {
        self.consume(); // if
        let condition = Box::new(self.parse_expr());
        let then_branch = Box::new(self.parse_block_expr());

        let else_branch = if matches!(self.peek(), Some(Token::Else)) {
            self.consume(); // else
            if matches!(self.peek(), Some(Token::If)) {
                Some(Box::new(self.parse_if_expr()))
            } else {
                Some(Box::new(self.parse_block_expr()))
            }
        } else {
            None
        };

        Expr::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_block_expr(&mut self) -> Expr {
        let mut stmts: Vec<Stmt> = Vec::new();

        self.consume(); // {

        while !matches!(self.peek(), Some(Token::RBrace)) {
            let stmt = self.parse_stmt();

            stmts.push(stmt);
        }

        self.consume(); // }

        Expr::Block(stmts)
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
            Some(Token::If) => self.parse_if_expr(),
            Some(Token::LBrace) => self.parse_block_expr(),
            Some(Token::Return) => self.parse_expr(),
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    fn parse_print(&mut self) -> Stmt {
        self.consume(); // print
        self.consume(); // (
        let expr = self.parse_expr();
        self.consume(); // )
        Stmt::Print(expr)
    }
}
