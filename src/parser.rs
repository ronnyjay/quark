use crate::{ast::{CallExprAST, ExprAST, FloatLiteralExprAST, IntegerLiteralExprAST, VariableExprAST }, lexer::{Lexeme, Token}};

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
    fn parse_prototype(&mut self) -> Option<Box<dyn ExprAST>> {
        if self.curtok().unwrap().token != Token::Identifier {
            println!("Expected function name in prototype");
        }

        todo!()
    }

    #[allow(dead_code)]
    fn parse_definition(&mut self) -> Option<Box<dyn ExprAST>> {
        todo!()
    }

    // Still not a fan of all the unwrapping and cloning
    // Will hopefully find a better way to handle this...
    fn parse_identifier_expr(&mut self) -> Option<Box<dyn ExprAST>> {
        let id_name = self.curtok().unwrap().value.clone().unwrap();

        self.nexttok(); // eat identifier 

        // todo: will panic if eof, fix.
        let tok1 = self.curtok().unwrap().value.clone().unwrap();
        if tok1 != "(" {
            return Some(Box::new(VariableExprAST { name: id_name }));
        }

        self.nexttok(); // eat (

        let mut _args: Vec<Box<dyn ExprAST>> = Vec::new();

        let tok2 = match self.curtok().and_then(|t| t.value.as_deref()) {
            Some(val) => val,
            None => {
                println!("\x1b[1;31merror:\x1b[0m Unexpected end of input in argument list");
                return None;
            }
        };

        if tok2 != ")" {
            loop {
                if let Some(arg) = self.parse_primary() {
                    _args.push(arg);
                }

                if self.curtok().unwrap().value.clone().unwrap() == ")" {
                    break;
                }

                if self.curtok().unwrap().value.clone().unwrap() != "," {
                    println!("Expected ')' or ',' in argument list");
                    break;
                }

                self.nexttok();
            }
        }

        self.nexttok(); // eat )

        return Some(Box::new(CallExprAST { function: id_name, args: _args }));
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