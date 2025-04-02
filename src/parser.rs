use crate::{ast::ExprAST, lexer::Lexeme};


#[allow(dead_code)]
pub struct CanonicalParser {
    pub expressions: Vec<Box<dyn ExprAST>>,
    pub lexemes: Vec<Lexeme>,
    pub pos: usize,
}

impl CanonicalParser {
    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        CanonicalParser { 
            expressions: Vec::new(), 
            lexemes, 
            pos: 0
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
}