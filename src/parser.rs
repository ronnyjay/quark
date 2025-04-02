use clap::{Error, ValueEnum};

use crate::{ast::{ExprAST, FloatLiteralExprAST, IntegerLiteralExprAST, VariableExprAST }, lexer::{Lexeme, Token}};

#[allow(dead_code)]
pub struct CanonicalParser {
    pub expressions: Vec<Box<dyn ExprAST>>,
    pub lexemes: Vec<Lexeme>,
    pub pos: usize,
}

impl CanonicalParser {
    // todo: lexemes could probably just be passed to process instead of being a member.
    // similarly, process could return expressions instead of them being stored here
    // - want to make this like the lexer where it's mostly 'static'
    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        CanonicalParser { 
            expressions: Vec::new(), 
            lexemes, 
            pos: 0
        }
    }

    // This implementation is purely for testing at the moment
    // If we are unable to process a token, simply skip to the next
    pub fn process(&mut self) {
        while self.pos < self.lexemes.len() {
            if let Some(expr) = self.parse_primary() {
                self.expressions.push(expr);
            } else {
                self.nexttok();
            }
        }
    }

    #[allow(dead_code)]
    fn curtok(&self) -> Option<&Lexeme> {
        self.lexemes.get(self.pos)
    }

    #[allow(dead_code)]
    fn nexttok(&mut self)  {
        self.pos += 1;
    }

    #[allow(dead_code)]
    fn parse_identifier_expr(&mut self) -> Option<Box<dyn ExprAST>> {
        // I think im just going to start unwrapping...
        let id_name = {
            if let Some(token) = self.curtok() {
                if let Some(value) = &token.value {
                    value.clone()
                }
                else {
                    return None;
                }
            }
            else {
                return None;
            }
        };

        self.nexttok(); // eat identifier 

        // Yeah, will definitely be unwrapping...
        let is_variable_ref: bool = {
            if let Some(token) = self.curtok() {
                if let Some(value) = &token.value {
                    if value != "(" {
                        true
                    }
                    else {
                        false
                    }
                } 
                else {
                    false
                }
            }
            else {
                false
            }
        };

        if is_variable_ref {
            return Some(Box::new(VariableExprAST { name: id_name }));
        }

        self.nexttok(); // eat (

        let mut _args: Vec<Box<dyn ExprAST>> = Vec::new();

        if let Some(token) = self.curtok() {
            if let Some(value) = &token.value {
                if value != ")" {
                    loop {
                        // todo: parse arguments
                        // will need to refactor this to allow for consuming nexttok

                        if self.curtok()?.value.as_ref()?.clone() == ")" {
                            break;
                        }

                        if self.curtok()?.value.as_ref()?.clone() != ","
                        {
                            println!("Expected ')' or ',' in argument list");
                            break;
                        }
                    }
                }
                else {
              
                }
            }
        }
        else {
            println!("Expected ')' or ',' in argument list");
        }

        return None;
    }

    #[allow(dead_code)]
    fn parse_literal_expr(&mut self) -> Option<Box<dyn ExprAST>> {
        if let Some(tok) = self.curtok() {
            match tok.token {
                Token::IntegerLiteral => {
                    if let Some(val) = &tok.value {
                        if let Ok(num) = val.parse::<i32>() {
                            self.nexttok();
                            return Some(Box::new(IntegerLiteralExprAST { value: num }));
                        }
                    }
                }
                Token::FloatLiteral => {
                    if let Some(val) = &tok.value {
                        if let Ok(num) = val.parse::<f32>() {
                            self.nexttok();
                            return Some(Box::new(FloatLiteralExprAST { value: num }));
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    #[allow(dead_code)]
    fn parse_primary(&mut self) -> Option<Box<dyn ExprAST>> {
        match &self.curtok() {
            Some(lex) => {
                match lex.token {
                  Token::IntegerLiteral | Token::FloatLiteral => {
                    self.parse_literal_expr()
                  },
                  Token::Identifier => {
                    self.parse_identifier_expr()
                  }
                  _ => None
                }
            }
            None => None
        }
    }
}