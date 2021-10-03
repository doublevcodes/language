use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use codespan_reporting::diagnostic::Label;
use codespan_reporting::files::SimpleFile;
use unicode_xid::UnicodeXID;

use crate::error::diagnostic::Error;
use crate::error::diagnostic::ErrorForm;
use crate::error::diagnostic::Errors;
use crate::token;
use crate::parser::token::{Token, TokenType};
use crate::parser::position::Position;

pub struct Lexer {
    source: Peekable<IntoIter<char>>,
    len: usize,
    keywords: HashMap<&'static str, TokenType>,
    pub tokens: Vec<Token>,
    pub pos: usize,
    pub line_number: usize,
    pub column_number: usize,
    pub error: Errors,
    pub file: SimpleFile<String, String>
}

impl Lexer {
    pub fn from_text(source: &str, file: SimpleFile<String, String>) -> Lexer {
        let keywords: HashMap<&str, TokenType> = HashMap::new();

        Lexer {
            source: source.chars().collect::<Vec<_>>().into_iter().peekable(),
            len: source.len(),
            keywords,
            tokens: Vec::new(),
            pos: 0,
            line_number: 1,
            column_number: 0,
            error: Errors::new(),
            file
        }
    }

    pub fn from_file(path: &str, file: SimpleFile<String, String>) -> io::Result<Lexer> {
        Ok(Lexer::from_text(&fs::read_to_string(path)?, file))
    }

    pub fn tokenise(&mut self) {
        let length = self.source.len();
        while self.pos <= length {
            if let Some(chr) = self.advance() {
                match chr {

                    ' ' | '\r' | '\t' => continue,
                    '\n' => self.advance_line(),
                    '#' => {
                        while self.source.peek() != Some(&'\n') && !self.at_end() {
                            self.advance();
                        }
                    },
                    
                    '+' => self.add_token(token!(+), "+"),
                    '-' => self.add_token(token!(-), "-"),
                    '*' => self.add_token(token!(*), "*"),
                    '/' => self.add_token(token!(/), "/"),
                    '%' => self.add_token(token!(%), "%"),
                    '^' => self.add_token(token!(^), "^"),

                    '&' => self.add_token(token!(&), "&"),
                    '|' => self.add_token(token!(|), "|"),
                    '~' => self.add_token(token!(~), "~"),
                    '@' => self.add_token(token!(@), "@"),
                    '<' => {
                        if self.possible_advance('<') {
                            self.add_token(token!(<<), "<<")
                        } else if self.possible_advance('=') {
                            self.add_token(token!(<=), "<=")
                        } else {
                            self.add_token(token!(<), "<")
                        }
                    },
                    '>' => {
                        if self.possible_advance('>') {
                            self.add_token(token!(>), ">>")
                        } else if self.possible_advance('=') {
                            self.add_token(token!(>=), ">=")
                        } else {
                            self.add_token(token!(>), ">")
                        }
                    },
                    '!' => {
                        if self.possible_advance('=') {
                            self.add_token(token!(!=), "!=")
                        } else {
                            let range = self.pos - 1..self.pos - 2;
                            let error = if !self.at_end() {
                                Error::new(
                                    ErrorForm::SyntaxError,
                                    format!(
                                        "I expected to find a '=', instead I recieved a {}.", self.source.peek().unwrap()
                                    ),
                                    vec![Label::primary((), range)],
                                    vec![String::from("Maybe you forgot the '=' after the '!'\nThis operator is the not equals operator and checks if two value are not the same.")]
                                )
                            } else {
                                Error::new(
                                    ErrorForm::EOFError,
                                    String::from("I expected to find a '=', instead the file ended!"),
                                    vec![Label::primary((), range)],
                                    vec![String::from("Seems like you ended the file in the middle of a comparison.\nConsider finishing it.")]
                                )
                            };

                            self.error.register_error(error);
                        }
                    },

                    '=' => {
                        if self.possible_advance('=') {
                            self.add_token(token!(==), "==")
                        } else {
                            self.add_token(token!(=), "=")
                        }
                    }

                    '"' => self.get_string(),

                    '0'..='9' => self.get_numeric(chr),
                    _ => {
                        if chr == '_' || UnicodeXID::is_xid_start(chr) {
                            self.get_identifier(chr);
                        }
                    }
                }
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.column_number += 1;
        self.source.next()
    }

    fn advance_line(&mut self) {
        self.line_number += 1;
        self.column_number = 0;
    }

    fn add_token(&mut self, form: TokenType, content: &str) {
        let position = Position {
            start: self.pos - content.len(),
            end: self.pos,
            line_number: self.line_number,
            column_number: self.column_number
        };

        let token = Token {
            form,
            content: String::from(content),
            position
        };

        self.tokens.push(token)
    }

    fn possible_advance(&mut self, expected: char) -> bool {
        if self.at_end() || self.source.peek().unwrap() != &expected {
            return false;
        }

        self.advance();
        true
    }

    fn at_end(&self) -> bool {
        self.pos >= self.len
    }

    fn get_string(&mut self) {
        let current_pos = self.pos - 1;
        let mut value = String::new();

        while self.source.peek() != Some(&'"') && !self.at_end() {
            if self.source.peek() == Some(&'\n') {
                self.advance_line();
            }
            let character = self.advance();
            value.push(character.unwrap());
        }

        if self.at_end() {
            let range = current_pos..self.pos;
            let error = Error::new(
                ErrorForm::EOFError,
                String::from("I expected you to close your string, but instead, the file ended!"),
                vec![Label::primary((), range)],
                vec![String::from("Add in a closing quotation mark to indicate that your string has ended?")]    
            );
            self.error.register_error(error);
        }

        self.advance();

        self.add_token(token!(string), &value)
    }

    fn get_numeric(&mut self, first: char) {
        let current_pos = self.pos - 1;
        let mut value = String::from(first);

        while let Some(c) = self.source.peek() {
            if c.is_numeric() {
                let chr = self.advance();
                value.push(chr.unwrap());

                if self.source.peek() == Some(&'.') {
                    let chr = self.advance();

                    if self.at_end() {
                        let range = self.pos..self.pos - 1;
                        let error = Error::new(
                            ErrorForm::EOFError, 
                            String::from("I found an incomplete float in your code"), 
                            vec![Label::primary((), range)],
                            vec![String::from("Make sure you finish typing out your float.")]
                        );

                        self.error.register_error(error);
                    }

                    if let Some(c) = self.source.peek() {
                        if c.is_numeric() {
                            let num_char = self.advance();
                            value.push(chr.unwrap());
                            value.push(num_char.unwrap());
                        } else {
                            let range = current_pos..self.pos;
                            let error = Error::new(
                                ErrorForm::EOFError, 
                                String::from("I found an invalid character inside this number"), 
                                vec![Label::primary((), range)],
                                vec![String::from("Make sure you finish typing out your float."), String::from("Make sure the 'invalid character' isn't a newline, it may trip you up!")]
                            );
    
                            self.error.register_error(error);
                        }
                    }
                }
            } else if c == &'.' {
                let char = self.advance();

                if self.at_end() {
                    let range = self.pos..self.pos;
                    let error = Error::new(
                        ErrorForm::EOFError, 
                        String::from("I found an incomplete float in your code"), 
                        vec![Label::primary((), range)],
                        vec![String::from("Make sure you finish typing out your float.")]
                    );

                    self.error.register_error(error);
                }

                if let Some(c) = self.source.peek() {
                    if c.is_numeric() {
                        let num_char = self.advance();
                        value.push(char.unwrap());
                        value.push(num_char.unwrap());
                    } else {
                        let range = current_pos..self.pos;
                        let error = Error::new(
                            ErrorForm::EOFError, 
                            String::from("I found an invalid character inside this number"), 
                            vec![Label::primary((), range)],
                            vec![String::from("Make sure you finish typing out your float.")]
                        );

                        self.error.register_error(error);
                    }
                }
            } else {
                break;
            }
        }

        let form: TokenType;
        
        if value.contains('.') {
            form = token!(float)
        } else {
            form = token!(integer)
        }

        self.add_token(form, &value)
    }

    fn get_identifier(&mut self, first_char: char) {
        let mut value = String::from(first_char);

        while let Some(c) = self.source.peek() {
            if c.is_alphanumeric() || c == &'_' {
                let chr = self.advance();
                value.push(chr.unwrap());
            } else {
                break;
            }
        }

        let form = match self.keywords.get(&value.as_str()) {
            Some(t) => t.clone(),
            None => token!(identifier),
        };

        self.add_token(form, &value)
    } 
}