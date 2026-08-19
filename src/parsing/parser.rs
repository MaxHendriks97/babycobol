use crate::{lexing::token::Token::{self, Evaluate}, parsing::ast::{DivideGiving, WhenExprStruct}};

use super::ast::{Atomic, DelimitedBy, DisplayVal, Expr, Identifier, Literal, Stmt, WhenClause, WhenClauseStruct};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn current(&self) -> Result<&Token, ParseError> {
        self.tokens.get(self.position)
            .ok_or(ParseError::UnexpectedToken("Unexpected end of input".into()))
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let token = self.current()?;
        let matches = match (&expected, token) {
            (Token::LeftParen(_), Token::LeftParen(_)) => true,
            (Token::RightParen(_), Token::RightParen(_)) => true,
            (Token::To(_), Token::To(_)) => true,
            (Token::Delimited(_), Token::Delimited(_)) => true,
            (Token::By(_), Token::By(_)) => true,
            (Token::Into(_), Token::Into(_)) => true,
            (Token::Also(_), Token::Also(_)) => true,
            (Token::When(_), Token::When(_)) => true,
            (Token::Other(_), Token::Other(_)) => true,
            (Token::Through(_), Token::Through(_)) => true,
            (Token::End(_), Token::End(_)) => true,
            _ => false,
        };

        if matches {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(format!("Expected {:?}, found {:?}", expected, token)))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while self.position < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let token = self.current()?;
        match token {
            Token::Accept(_) => self.parse_accept(),
            Token::Add(_) => self.parse_add(),
            Token::Display(_) => self.parse_display(),
            Token::Divide(_) => self.parse_divide(),
            Token::Evaluate(_) => self.parse_evaluate(),
            _ => Err(ParseError::UnexpectedToken(format!("Expected statement, found {:?}", token))),
        }
    }

    fn parse_accept(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let mut identifiers: Vec<Identifier> = vec!();

        // Parse identifiers
        while let Ok(token) = self.current() {
            match token {
                Token::Identifier(val) => identifiers.push(Identifier(val.to_string())),
                _ => break,
            }
            self.advance()
        }

        Ok(Stmt::Accept(identifiers))
    }

    fn parse_add(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let mut values: Vec<Atomic> = vec!();

        while let Ok(token) = self.current() {
            match token {
                Token::IntegerLiteral(val) => values.push(Atomic::Literal(Literal::IntegerLiteral(val.clone()))),
                Token::StringLiteral(val) => values.push(Atomic::Literal(Literal::StringLiteral(val.clone()))),
                Token::BooleanLiteral(val) => values.push(Atomic::Literal(Literal::BooleanLiteral(val.clone()))),
                Token::Identifier(val) => values.push(Atomic::Identifier(Identifier(val.clone()))),
                _ => break,
            }
            self.advance();
        }

        if values.len() == 0 {
            return Err(ParseError::UnexpectedToken(
                format!("Expected at least one identifier, found {:?}", self.current()?)
            ));
        }

        self.expect(Token::To("".to_string()))?;

        let target: Atomic = self.parse_atomic()?;

        let mut giving = None;
        if let Ok(Token::Giving(_)) = self.current() {
            let mut identifiers: Vec<Identifier> = vec!();
            self.advance();

            while let Ok(Token::Identifier(val)) = self.current() {
                identifiers.push(Identifier(val.to_string()));
                self.advance();
            }

            if identifiers.len() == 0 {
                return Err(ParseError::UnexpectedToken(
                    format!("Expected at least one identifier, found {:?}", self.current()?)
                ));
            }

            giving = Some(identifiers);
        }

        Ok(Stmt::Add {
            values,
            target,
            giving,
        })
    }

    fn parse_display(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let mut values: Vec<DisplayVal> = vec!();

        while let Ok(atomic) = self.parse_atomic() {
            let mut delimited_by: Option<DelimitedBy> = None;

            if let Ok(_) = self.expect(Token::Delimited("".to_string())) {
                self.expect(Token::By("".to_string()))?;

                delimited_by = Some(match self.current()? {
                    Token::Size(_) => DelimitedBy::Size,
                    Token::Space(_) => DelimitedBy::Space,
                    Token::StringLiteral(val) => DelimitedBy::Literal(Literal::StringLiteral(val.clone())),
                    Token::IntegerLiteral(val) => DelimitedBy::Literal(Literal::IntegerLiteral(val.clone())),
                    Token::BooleanLiteral(val) => DelimitedBy::Literal(Literal::BooleanLiteral(val.clone())),
                    _ => return Err(ParseError::UnexpectedToken(
                        format!("Expected \"SIZE\", \"SPACE\" or a literal, found {:?}", self.current()?)
                    )),
                });
            }

            values.push(DisplayVal {
                value: atomic,
                delimited_by,
            })
        }

        let mut with_no_advancing: bool = false;
        if let Ok(_) = self.expect(Token::With("".to_string())) {
            self.expect(Token::No("".to_string()))?;
            self.expect(Token::Advancing("".to_string()))?;
            with_no_advancing = true;
        }

        Ok(Stmt::Display {
            values,
            with_no_advancing,
        })
    }

    fn parse_divide(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let value: Atomic = self.parse_atomic()?;

        self.expect(Token::Into("".to_string()))?;

        let mut into: Vec<Atomic> = vec!();
        while let Ok(atomic) = self.parse_atomic() {
            into.push(atomic);
        }

        if into.len() == 0 {
            return Err(ParseError::UnexpectedToken(format!("Expected at least one atomic, found {:?}", self.current()?)));
        }

        let mut divide_giving: Option<DivideGiving> = None;
        if let Ok(_) = self.expect(Token::Giving("".to_string())) {
            let mut giving: Vec<Identifier> = vec!();
            while let Ok(Token::Identifier(val)) = self.current() {
                giving.push(Identifier(val.clone()));
                self.advance();
            }

            if giving.len() == 0 {
                return Err(ParseError::UnexpectedToken(format!("Expected at least one identifier, found {:?}", self.current()?)));
            }

            let mut remainder: Option<Identifier> = None;
            if let Ok(_) = self.expect(Token::Remainder("".to_string())) {
                remainder = match self.current()? {
                    Token::Identifier(val) => Some(Identifier(val.clone())),
                    _ => return Err(ParseError::UnexpectedToken(format!("Expected Identifier, found {:?}", self.current()?))),
                };
            }

            divide_giving = Some(DivideGiving{
                giving,
                remainder,
            });
        }

        Ok(Stmt::Divide {
            value,
            into,
            divide_giving,
        })
    }

    fn parse_evaluate(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let expression = self.parse_expression()?;

        let mut also: Vec<Expr> = vec!();
        while let Ok(_) = self.expect(Token::Also("".to_string())) {
            let expr = self.parse_expression()?;
            also.push(expr);
        }

        let mut when: Vec<(WhenClause, Vec<Stmt>)> = vec!();
        while let Ok(when_clause) = self.parse_when_clause() {
            let mut stmts: Vec<Stmt> = vec!();
            while let Ok(stmt) = self.parse_statement() {
                stmts.push(stmt);
            }

            when.push((when_clause, stmts));
        }

        self.expect(Token::End("".to_string()))?;

        Ok(Stmt::Evaluate {
            expression,
            also,
            when,
        })
    }

    fn parse_when_clause(&mut self) -> Result<WhenClause, ParseError> {
        self.expect(Token::When("".to_string()))?;
        if let Ok(_) = self.expect(Token::Other("".to_string())) {
            return Ok(WhenClause::Other);
        }

        let mut when_exprs: Vec<WhenExprStruct> = vec!();
        while let Ok(when_expr) = self.parse_when_expr() {
            when_exprs.push(when_expr);
        }

        if when_exprs.len() == 0 {
            return Err(ParseError::UnexpectedToken(format!("Expected at least one expression, found {:?}", self.current()?)));
        }

        let mut also_exprs: Vec<Vec<WhenExprStruct>> = vec!();
        while let Ok(_) = self.expect(Token::Also("".to_string())) {
            let mut also_expr_exprs: Vec<WhenExprStruct> = vec!();

            while let Ok(when_expr) = self.parse_when_expr() {
                also_expr_exprs.push(when_expr);
            }

            also_exprs.push(also_expr_exprs);
        }

        Ok(WhenClause::Expr(WhenClauseStruct {
            when_exprs,
            also_exprs,
        }))
    }

    fn parse_when_expr(&mut self) -> Result<WhenExprStruct, ParseError> {
        let expr = self.parse_expression()?;
        let mut through: Option<Expr> = None;
        if let Ok(_) = self.expect(Token::Through("".to_string())) {
            through = Some(self.parse_expression()?);
        }

        Ok(WhenExprStruct {
            expr,
            through,
        })
    }

    fn parse_atomic(&mut self) -> Result<Atomic, ParseError> {
        let atomic = match self.current()? {
            Token::IntegerLiteral(val) => Atomic::Literal(Literal::IntegerLiteral(val.clone())),
            Token::StringLiteral(val) => Atomic::Literal(Literal::StringLiteral(val.clone())),
            Token::BooleanLiteral(val) => Atomic::Literal(Literal::BooleanLiteral(val.clone())),
            Token::Identifier(val) => Atomic::Identifier(Identifier(val.clone())),
            _ => return Err(ParseError::UnexpectedToken(
                format!("Expected Atomic, found {:?}", self.current()?)
            )),
        };

        self.advance();

        Ok(atomic)
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        if let Ok(atomic) = self.parse_atomic() {
            return Ok(Expr::Atomic(atomic));
        }

        Err(ParseError::UnexpectedToken(format!("Expected expression, found {:?}", self.current()?)))
    }

}

pub enum ParseError {
    UnexpectedToken(String),
}
