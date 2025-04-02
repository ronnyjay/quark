use crate::{ast::{ExprAST, FloatLiteralExprAST, IntegerLiteralExprAST }, lexer::{Lexeme, Token}};

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
                  _ => None
                }
            }
            None => None
        }
    }
}