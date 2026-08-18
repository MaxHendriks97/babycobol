use crate::lexing::token::Token;

use super::ast::{Atomic, BooleanExpr, Identifier, Stmt};

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
                Token::IntegerLiteral(val) => values.push(Atomic::IntegerLiteral(val.clone())),
                Token::StringLiteral(val) => values.push(Atomic::StringLiteral(val.clone())),
                Token::BooleanLiteral(val) => values.push(Atomic::BooleanLiteral(BooleanExpr(val.clone()))),
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

        let target: Atomic = match self.current()? {
            Token::IntegerLiteral(val) => Atomic::IntegerLiteral(val.clone()),
            Token::StringLiteral(val) => Atomic::StringLiteral(val.clone()),
            Token::BooleanLiteral(val) => Atomic::BooleanLiteral(BooleanExpr(val.clone())),
            Token::Identifier(val) => Atomic::Identifier(Identifier(val.clone())),
            _ => return Err(ParseError::UnexpectedToken(format!("Expected Atomic, found {:?}", self.current()?))),
        };
        self.advance();

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
}

pub enum ParseError {
    UnexpectedToken(String),
}
