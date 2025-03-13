use std::{
    collections::LinkedList,
    fmt::{self, format},
};

#[derive(Debug)]
pub enum Token {
    Identifier,

    Plus,
    Minus,
    Equals,
    Asterisk,
    ForwardSlash,

    PlusEquals,
    MinusEquals,
    EqualsEquals,
    AsteriskEquals,
    ForwardSlashEquals,

    PlusPlus,
    MinusMinus,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,
    LeftCaret,
    RightCaret,
}

pub struct Lexeme {
    pub line_number: usize,
    pub token: Token,
    pub value: Option<String>,
}

impl fmt::Debug for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}: {:?} {}", self.line_number, self.token, value)
        } else {
            write!(f, "{}: {:?}", self.line_number, self.token)
        }
    }
}

#[derive(PartialEq)]
enum LexerState {
    Overall,
    Identifier,
    Number,
    Literal,
    Operator,
}

fn is_letter(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z')
}

fn is_number(c: u8) -> bool {
    matches!(c, b'1'..=b'9')
}

fn is_start_of_identifier(c: u8) -> bool {
    is_letter(c)
}

fn is_valid_in_identifier(c: u8) -> bool {
    is_letter(c) | is_number(c) | matches!(c, b'_')
}

fn is_whitespace(c: u8) -> bool {
    matches!(c, b' ' | b'\t')
}

fn is_operator(c: u8) -> bool {
    matches!(
        c,
        b'{' | b'}'
            | b'['
            | b']'
            | b'+'
            | b'-'
            | b'*'
            | b'/'
            | b'&'
            | b'|'
            | b'.'
            | b'<'
            | b'>'
            | b'^'
    )
}

fn string_to_operator(str: &String) -> Option<Token> {
    match str.as_str() {
        "+=" => Some(Token::PlusEquals),
        "-=" => Some(Token::MinusEquals),
        "*=" => Some(Token::AsteriskEquals),
        "/=" => Some(Token::ForwardSlashEquals),
        "==" => Some(Token::EqualsEquals),
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "++" => Some(Token::PlusPlus),
        "--" => Some(Token::MinusMinus),
        "*" => Some(Token::Asterisk),
        "/" => Some(Token::ForwardSlash),
        "=" => Some(Token::Equals),
        "{" => Some(Token::LeftBracket),
        "}" => Some(Token::RightBracket),
        "[" => Some(Token::LeftBrace),
        "]" => Some(Token::RightBrace),
        "<" => Some(Token::LeftCaret),
        ">" => Some(Token::RightCaret),
        _ => None,
    }
}

pub fn process(bytes: &Vec<u8>) -> Result<Vec<Lexeme>, String> {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    let mut states: LinkedList<LexerState> = LinkedList::new();
    states.push_back(LexerState::Overall);

    let mut line_number: usize = 1;
    let mut value = String::new();

    for byte in bytes {
        let state = match states.back() {
            Some(state) => state,
            None => return Err(format!("Internal parsing error.")),
        };

        match state {
            LexerState::Overall => {
                if *byte == b'\n' {
                    line_number += 1;
                } else if is_whitespace(*byte) {
                } else if is_start_of_identifier(*byte) {
                    states.push_back(LexerState::Identifier);
                    value.push(*byte as char);
                } else if is_operator(*byte) {
                    states.push_back(LexerState::Operator);
                    value.push(*byte as char);
                } else if *byte == b'\0' {
                    break;
                } else {
                    return Err(format!(
                        "Unexpected token on line {}: '{}'",
                        line_number, *byte as char
                    ));
                }
            }
            LexerState::Identifier => {
                if is_valid_in_identifier(*byte) {
                    value.push(*byte as char);
                } else if is_whitespace(*byte) || *byte == b'\n' || *byte == b'\0' {
                    states.pop_back();
                    lexemes.push(Lexeme {
                        line_number,
                        token: Token::Identifier,
                        value: Some(value.clone()),
                    });
                    value.clear();

                    if *byte == b'\n' {
                        line_number += 1;
                    }
                } else {
                    return Err(format!(
                        "Unexpected token on line {}: '{}'",
                        line_number, *byte as char
                    ));
                }
            }
            LexerState::Operator => {
                if is_whitespace(*byte) || *byte == b'\n' || *byte == b'\0' {
                    if let Some(op) = string_to_operator(&value) {
                        states.pop_back();
                        lexemes.push(Lexeme {
                            line_number,
                            token: op,
                            value: Some(value.clone()),
                        });
                        value.clear();

                        if *byte == b'\n' {
                            line_number += 1;
                        }
                    } else {
                        return Err(format!(
                            "Unexpected token on line {}: \n\
                            Unrecognized operator: {}",
                            line_number, value
                        ));
                    }
                } else {
                    value.push(*byte as char);
                }
            }
            _ => return Err(format!("Internal parsing error.")),
        };
    }

    if let Some(state) = states.pop_back() {
        if state != LexerState::Overall {
            return Err(format!("Expected lexer state to be empty"));
        }
    }

    Ok(lexemes)
}
