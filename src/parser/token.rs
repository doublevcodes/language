use crate::parser::position::Position;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    INTEGER,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub form: TokenType,
    pub position: Position,
    pub content: String,
}

impl Token {
    pub fn len(&self) -> usize {
        self.content.len()
    }
}

#[macro_export]
macro_rules! token {
    [+] => {
        $crate::parser::token::TokenType::PLUS
    };
    [-] => {
        $crate::parser::token::TokenType::MINUS
    };
    [*] => {
        $crate::parser::token::TokenType::MULTIPLY
    };
    [/] => {
        $crate::parser::token::TokenType::DIVIDE
    };
    [integer] => {
        $crate::parser::token::TokenType::INTEGER
    };
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                token!(+) => "+",
                token!(-) => "-",
                token!(*) => "*",
                token!(/) => "/",
                token!(integer) => "integer"
            }
        )
    }
}