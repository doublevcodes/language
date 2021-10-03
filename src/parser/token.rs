use crate::parser::position::Position;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {

    // Values
    Integer,
    Float,
    String,
    True,
    False,

    // Identifier
    Identifier,

    // Assignment
    Assignment,
    
    // Arithmetic ops
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Exponent,

    // Bitwise ops
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLshift,
    BitwiseRshift,

    // Relational ops
    Equals,
    NotEquals,
    LessThan,
    LessEquals,
    GreaterThan,
    GreaterEquals,

    // Miscellaneous
    Hash,
    Eof
}

#[derive(PartialEq)]
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} with value '{}' at {}:{})", self.form, self.content, self.position.line_number, self.position.column_number)
    }
}

#[macro_export]
macro_rules! token {
    [+] => {
        $crate::parser::token::TokenType::Plus
    };
    [-] => {
        $crate::parser::token::TokenType::Minus
    };
    [*] => {
        $crate::parser::token::TokenType::Multiply
    };
    [/] => {
        $crate::parser::token::TokenType::Divide
    };
    [%] => {
        $crate::parser::token::TokenType::Modulus
    };
    [^] => {
        $crate::parser::token::TokenType::Exponent
    };
    [&] => {
        $crate::parser::token::TokenType::BitwiseAnd
    };
    [|] => {
        $crate::parser::token::TokenType::BitwiseOr
    };
    [~] => {
        $crate::parser::token::TokenType::BitwiseNot
    };
    [@] => {
        $crate::parser::token::TokenType::BitwiseXor
    };
    [<<] => {
        $crate::parser::token::TokenType::BitwiseLshift
    };
    [>>] => {
        $crate::parser::token::TokenType::BitwiseRshift
    };
    [==] => {
        $crate::parser::token::TokenType::Equals
    };
    [!=] => {
        $crate::parser::token::TokenType::NotEquals
    };
    [<] => {
        $crate::parser::token::TokenType::LessThan
    };
    [<=] => {
        $crate::parser::token::TokenType::LessEquals
    };
    [>] => {
        $crate::parser::token::TokenType::GreaterThan
    };
    [>=] => {
        $crate::parser::token::TokenType::GreaterEquals
    };
    [integer] => {
        $crate::parser::token::TokenType::Integer
    };
    [float] => {
        $crate::parser::token::TokenType::Float
    };
    [string] => {
        $crate::parser::token::TokenType::String
    };
    [true] => {
        $crate::parser::token::TokenType::True
    };
    [false] => {
        $crate::parser::token::TokenType::False
    };
    [identifier] => {
        $crate::parser::token::TokenType::Identifier
    };
    [=] => {
        $crate::parser::token::TokenType::Assignment
    };
    [#] => {
        $crate::parser::token::TokenType::Hash
    };
    [eof] => {
        $crate::parser::token::TokenType::Eof
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
                token!(%) => "%",
                token!(^) => "^",
                token!(&) => "&",
                token!(|) => "|",
                token!(~) => "~",
                token!(@) => "@",
                token!(<<) => "<<",
                token!(>>) => ">>",
                token!(==) => "==",
                token!(!=) => "!=",
                token!(<) => "<",
                token!(<=) => "<=",
                token!(>) => ">",
                token!(>=) => ">=",
                token!(integer) => "integer",
                token!(float) => "float",
                token!(string) => "string",
                token!(true) => "true",
                token!(false) => "false",
                token!(identifier) => "identifier",
                token!(=) => "=",
                token!(#) => "#",
                token!(eof) => "eof"
            }
        )
    }
}