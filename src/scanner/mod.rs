pub(crate) mod token;

use std::collections::HashMap;

use crate::error::{LoxError, SyntaxError};
use token::{Object, Token, TokenType};
// use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: &String) -> Scanner {
        let keywords = [
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]
        .iter()
        .cloned()
        .collect();

        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        // let mut had_error: Option<LoxError> = None;
        let mut source_errors = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err((err_msg, help_msg)) => {
                    source_errors.push(SyntaxError {
                        line: self.line,
                        span: (self.start, self.current).into(),
                        err_message: err_msg,
                        help_message: help_msg,
                    });
                    // TODO: implement error using `miette` library
                    // e.report("".to_string());
                    // had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.line,
            (self.start, self.current),
        ));

        if !source_errors.is_empty() {
            return Err(LoxError::SyntaxErrors(source_errors));
        } else {
            Ok(&self.tokens)
        }
        // if let Some(e) = had_error {
        //     Err(e)
        // } else {
        //     Ok(&self.tokens)
        // }
    }

    fn scan_token(&mut self) -> Result<(), (String, String)> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // for matching `!` or `!=`
            '!' => {
                let ty = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(ty);
            }

            // for matching `!` or `==`
            '=' => {
                let ty = if self.is_match('=') {
                    TokenType::Equals
                } else {
                    TokenType::Assign
                };
                self.add_token(ty);
            }

            // for matching `!` or `<=`
            '<' => {
                let ty = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(ty);
            }

            // for matching `!` or `>=`
            '>' => {
                let ty = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(ty);
            }

            // for matching '/' or '//' i.e for division of commenting
            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                /* else if self.is_match('*') {
                    // multi line comment start
                    self.scan_comment();
                }*/
                else {
                    self.add_token(TokenType::Slash)
                }
            }

            ' ' | '\r' | '\t' => {
                // ignore whitespaces
            }

            '\n' => {
                self.line += 1;
            }

            '"' => {
                _ = self.handle_string();
            }

            /*
                a if a.is_alphabetic() || a == '_' => {
                self.handle_string();
                }
                d if d.is_ascii_digit() => {
                self.handle_number();
                }
            */
            _ => {
                if c.is_ascii_digit() {
                    _ = self.handle_number();
                } else if self.is_alpha(c) {
                    self.handle_identifier();
                } else {
                    // return Err(LoxError::error(
                    //     self.line,
                    //     "Unexpected character".to_string(),
                    // ));
                    return Err((
                        "unexpected character".to_string(),
                        "try removing the character".to_string(),
                    ));
                }
                // }
            }
        }
        Ok(())
    }

    // fn scan_comment(&mut self) {
    //     let mut nesting = 1;

    //     while self.peek() != '*' {
    //         if self.is_match('/') {
    //             break;
    //         }
    //         self.advance();
    //     }
    // }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_obj(ttype, None);
    }

    fn add_token_obj(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        // self.tokens
        //     .push(Token::new(ttype, lexeme, literal, self.line))
        self.tokens.push(Token::new(
            ttype,
            lexeme,
            literal,
            self.line,
            (self.start, self.current),
        ))
    }

    fn handle_string(&mut self) -> Result<(), (String, String)> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err((
                "unterminated string".to_string(),
                "add a '\"' after the string".to_string(),
            ));

            // LoxError::error(self.line, "Unterminated string.".to_string());
            // return;
        }

        // the closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_obj(TokenType::String, Some(Object::Str(value)));

        Ok(())
    }

    fn handle_number(&mut self) -> Result<(), (String, String)> {
        /*
        while self.is_digit(self.peek()) {
            self.advance();
        }
        */
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit()
        /*self.is_digit(self.peek_next())*/
        {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit()
            /* self.is_digit(self.peek())*/
            {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        match value.parse::<f64>() {
            Ok(num) => {
                self.add_token_obj(TokenType::Number, Some(Object::Num(num)));
                Ok(())
            }
            Err(_) => Err((
                "couldn't parse the number".to_string(),
                "make sure you are only using integers or floating point numbers".to_string(),
            )),
        }
    }

    fn handle_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let identifier: String = self.source[self.start..self.current].iter().collect();
        let ttype = match self.keywords.get(&identifier.as_str()).cloned() {
            Some(token) => token,
            None => TokenType::Identifier,
        };
        self.add_token(ttype);
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.char_at(self.current) != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let current_char = self.char_at(self.current);
        self.current += 1;
        current_char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.char_at(self.current);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.char_at(self.current + 1)
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit() /* self.is_digit(c) */
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn char_at(&self, index: usize) -> char {
        self.source[index]
    }
}
