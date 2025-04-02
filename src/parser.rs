use crate::{ast::ExprAST, lexer::{Lexeme, Token}};

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

    #[allow(dead_code)]
    fn parse_primary(&self) -> Option<Box<dyn ExprAST>> {
        match &self.curtok() {
            Some(lex) => {
                match lex.token {
                    Token::Identifier => todo!(),
                    Token::Number => todo!(),
                    Token::Plus => todo!(),
                    Token::Minus => todo!(),
                    Token::Equals => todo!(),
                    Token::Asterisk => todo!(),
                    Token::AsteriskAsterisk => todo!(),
                    Token::Percent => todo!(),
                    Token::Ampersand => todo!(),
                    Token::AmpersandAmpersand => todo!(),
                    Token::Pipe => todo!(),
                    Token::PipePipe => todo!(),
                    Token::ForwardSlash => todo!(),
                    Token::PlusEquals => todo!(),
                    Token::MinusEquals => todo!(),
                    Token::EqualsEquals => todo!(),
                    Token::AsteriskEquals => todo!(),
                    Token::ForwardSlashEquals => todo!(),
                    Token::ForwardSlashForwardSlash => todo!(),
                    Token::PlusPlus => todo!(),
                    Token::MinusMinus => todo!(),
                    Token::LeftParenthesis => todo!(),
                    Token::RightParenthesis => todo!(),
                    Token::LeftBracket => todo!(),
                    Token::RightBracket => todo!(),
                    Token::LeftBrace => todo!(),
                    Token::RightBrace => todo!(),
                    Token::LeftCaret => todo!(),
                    Token::RightCaret => todo!(),
                    Token::LeftCaretLeftCaret => todo!(),
                    Token::RightCaretRightCaret => todo!(),
                    Token::LeftCaretEquals => todo!(),
                    Token::RightCaretEquals => todo!(),
                    Token::Semicolon => todo!(),
                    Token::ColonColon => todo!(),
                    Token::IntegerLiteral => todo!(),
                    Token::FloatLiteral => todo!(),
                    Token::CharLiteral => todo!(),
                }
            }
            None => None
        }
    }
}