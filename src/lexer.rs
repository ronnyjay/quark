use std::{
    collections::{LinkedList, VecDeque},
    fmt::{self, format},
};

#[derive(Debug)]
pub enum Token {
    Identifier,
    Number,

    Plus,
    Minus,
    Equals,
    Asterisk,
    AsteriskAsterisk,
    Percent,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,

    ForwardSlash,

    PlusEquals,
    MinusEquals,
    EqualsEquals,
    AsteriskEquals,
    ForwardSlashEquals,
    ForwardSlashForwardSlash,

    PlusPlus,
    MinusMinus,

    LeftParenthesis,
    RightParenthesis,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    LeftCaret,
    RightCaret,

    LeftCaretLeftCaret,
    RightCaretRightCaret,

    LeftCaretEquals,
    RightCaretEquals,

    Semicolon,

    ColonColon, // :: import statements, scope resolution?

    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
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

#[derive(PartialEq, Debug)]
enum LexerState {
    Overall,
    Identifier,

    WholeNumber,   // -100000
    Decimal,       // .6002
    Exponentional, // e+7, E-09

    Literal,
    Operator,
}

fn is_letter(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z')
}

fn is_number(c: u8) -> bool {
    return c.is_ascii_digit();
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
            | b'('
            | b')'
            | b'+'
            | b'-'
            | b'='
            | b'*'
            | b'/'
            | b'&'
            | b'|'
            | b'.'
            | b'<'
            | b'>'
            | b'^'
            | b':'
            | b';'
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
        "**" => Some(Token::AsteriskAsterisk),
        "&" => Some(Token::Ampersand),
        "&&" => Some(Token::AmpersandAmpersand),
        "|" => Some(Token::Pipe),
        "||" => Some(Token::PipePipe),
        "%" => Some(Token::Percent),
        "/" => Some(Token::ForwardSlash),
        "=" => Some(Token::Equals),
        "(" => Some(Token::LeftParenthesis),
        ")" => Some(Token::RightParenthesis),
        "{" => Some(Token::LeftBracket),
        "}" => Some(Token::RightBracket),
        "[" => Some(Token::LeftBrace),
        "]" => Some(Token::RightBrace),
        "<" => Some(Token::LeftCaret),
        ">" => Some(Token::RightCaret),
        ";" => Some(Token::Semicolon),
        "::" => Some(Token::ColonColon),
        _ => None,
    }
}

pub fn process(bytes: &mut VecDeque<u8>) -> Result<Vec<Lexeme>, String> {
    let mut lexemes: Vec<Lexeme> = Vec::new();
    let mut state: LexerState = LexerState::Overall;

    let mut line_number: usize = 1;
    let mut value = String::new();

    while !bytes.is_empty() {
        let byte = match bytes.front() {
            Some(byte) => byte,
            None => return Err(format!("We shouldn't really get here?")),
        };

        match state {
            LexerState::Overall => {
                if *byte == b'\n' {
                    line_number += 1;
                    bytes.pop_front();
                } else if is_whitespace(*byte) {
                    bytes.pop_front();
                } else if is_start_of_identifier(*byte) {
                    state = LexerState::Identifier;
                } else if *byte == b'\0' {
                    break;
                } else {
                    let current = *byte;
                    bytes.pop_front();

                    if let Some(lookahead) = bytes.front() {
                        if lookahead.is_ascii_digit() {
                            bytes.push_front(current);
                            state = LexerState::WholeNumber;
                        } else {
                            bytes.push_front(current);
                            state = LexerState::Operator;
                        }
                    } else {
                        bytes.push_front(current);
                        state = LexerState::Operator;
                    }
                }
            }
            LexerState::Identifier => {
                if is_valid_in_identifier(*byte) {
                    value.push(*byte as char);
                    bytes.pop_front();
                } else {
                    state = LexerState::Overall;
                    lexemes.push(Lexeme {
                        line_number,
                        token: Token::Identifier,
                        value: Some(value.clone()),
                    });
                    value.clear();
                }
            }
            LexerState::Operator => {
                let mut token = None;
                let current = match bytes.pop_front() {
                    Some(current) => current,
                    None => return Err(format!("Got an empty operator? Line {}", line_number)),
                };

                value.push(current as char);
                let single_op = string_to_operator(&value);

                match bytes.front() {
                    Some(next) => {
                        value.push(*next as char);
                        let double_op = string_to_operator(&value);
                        if let Some(_) = double_op {
                            token = double_op;
                            bytes.pop_front();
                        } else {
                            token = single_op;
                            value.pop();
                        }
                    }
                    None => token = single_op,
                };

                if let Some(token) = token {
                    lexemes.push(Lexeme {
                        line_number,
                        token,
                        value: Some(value.clone()),
                    });
                    value.clear();
                    state = LexerState::Overall;
                }
            }
            LexerState::WholeNumber => {
                match *byte {
                    b'-' | b'+' => {
                        if value.is_empty() {
                            value.push(*byte as char);
                            bytes.pop_front();
                        } else {
                            return Err(format!(
                                "Unexpected token on line {}: {}",
                                line_number, byte
                            ));
                        }
                    }
                    b'0'..b'9' => {
                        value.push(*byte as char);
                        bytes.pop_front();
                    }
                    b'.' => {
                        value.push(*byte as char);
                        bytes.pop_front();
                        state = LexerState::Decimal;
                    }
                    b'e' | b'E' => {
                        value.push(*byte as char);
                        bytes.pop_front();

                        // only one + or - can occur after e/E
                        // check for +/- here to simplify logic

                        match bytes.front() {
                            Some(c) => {
                                if *c == b'-' || *c == b'+' {
                                    value.push(*c as char);
                                    bytes.pop_front();
                                }
                            }
                            _ => {}
                        }

                        state = LexerState::Exponentional;
                    }
                    _ => {
                        state = LexerState::Overall;
                        lexemes.push(Lexeme {
                            line_number,
                            token: Token::IntegerLiteral,
                            value: Some(value.clone()),
                        });
                        value.clear();
                    }
                }
            }
            LexerState::Decimal => match *byte {
                b'0'..b'9' => {
                    value.push(*byte as char);
                    bytes.pop_front();
                }
                b'e' | b'E' => {
                    value.push(*byte as char);
                    bytes.pop_front();

                    // only one + or - can occur after e/E
                    // check for +/- here to simplify logic

                    match bytes.front() {
                        Some(c) => {
                            if *c == b'-' || *c == b'+' {
                                value.push(*c as char);
                                bytes.pop_front();
                            }
                        }
                        _ => {}
                    }

                    state = LexerState::Exponentional;
                }
                _ => {
                    state = LexerState::Overall;
                    lexemes.push(Lexeme {
                        line_number,
                        token: Token::FloatLiteral,
                        value: Some(value.clone()),
                    });
                    value.clear();
                }
            },
            LexerState::Exponentional => match *byte {
                b'0'..=b'9' => {
                    value.push(*byte as char);
                    bytes.pop_front();
                }
                _ => {
                    state = LexerState::Overall;
                    lexemes.push(Lexeme {
                        line_number,
                        token: Token::FloatLiteral,
                        value: Some(value.clone()),
                    });
                    value.clear();
                }
            },
            _ => return Err(format!("Internal parsing error.")),
        };
    }

    if state != LexerState::Overall {
        return Err(format!("Expected lexer state to be empty: {:?}", state));
    }

    Ok(lexemes)
}
