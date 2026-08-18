use crate::lexing::token::Token;

use super::ast::{Atomic, DelimitedBy, DisplayVal, Identifier, Literal, Stmt};

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
}

pub enum ParseError {
    UnexpectedToken(String),
}
