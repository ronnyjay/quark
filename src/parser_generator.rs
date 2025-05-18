use std::{vec::IntoIter, iter::Peekable};

use crate::{ast::{CallExprAST, ExprAST, VariableExprAST}, lexer::{Lexeme, Token}};

#[allow(dead_code)]
pub struct ParserGenerator {
    pub lexemes: Peekable<IntoIter<Lexeme>>,
    pub expressions: Vec<Box<dyn ExprAST>>
}

#[allow(dead_code)]
impl ParserGenerator {
    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        Self {
            lexemes: lexemes.into_iter().peekable(),
            expressions: Vec::new()
        }
    }

    pub fn print(&self) {
        for expression in &self.expressions {
            expression.print();
        }
    }

    pub fn curtok(&mut self) -> Option<&Lexeme> {
        self.lexemes.peek()
    }

    pub fn gettok(&mut self) -> Option<Lexeme> {
        self.lexemes.next()
    }

    pub fn process(&mut self) {
        while let Some(lexeme) = self.curtok() {
            match lexeme.token {
                Token::Identifier => {
                    if let Some(expression) = self.process_identifier() {
                        self.expressions.push(expression);
                    }
                },
                Token::Keyword(_) => self.process_keyword(),
                _ => {
                    self.gettok(); // Consume next token, we're not parsing it right now
                    continue;
                }
            }
        }
    }

    pub fn process_keyword(&mut self) {
        let keyword = self.gettok().unwrap().value;

        println!("Processing keyword: {}", keyword.as_ref().unwrap());

        // Function declaration/definition
        if keyword.as_ref().unwrap() == "fn" {
        }
        // Variable declaration/definition
        else {
        }

    }

    pub fn process_identifier(&mut self) -> Option<Box<dyn ExprAST>> {
        let identifier = self.gettok().unwrap().value;

        println!("Processing identifier {}", identifier.as_ref().unwrap());

        // Simple variable reference
        if self.curtok().unwrap().value.as_ref().unwrap().as_str() != "(" { 
            return Some(Box::new(VariableExprAST {name: identifier.unwrap() }));
        }
        
        self.gettok(); // Eat (

        // Call expression
        if self.curtok().unwrap().value.as_ref().unwrap().as_str() != ")" {
            loop {

                // TODO: Parse arguments

                if self.curtok().unwrap().value.as_ref().unwrap().as_str() == ")" {
                    break;
                }
            }
        }

        self.gettok(); // Eat )

        return Some(Box::new(CallExprAST{function: identifier.unwrap(), args: Vec::new()}));
    }  
}
