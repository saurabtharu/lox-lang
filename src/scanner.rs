use crate::error::LoxError;
use crate::token::Token;
use crate::token_type::TokenType;
use substring::Substring;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens();
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    //     fn scan_token(&self) {
    //         let c = self.advance();

    //         match c {
    //             '(' => self.add_token(TokenType::LeftParam),
    //             ')' => self.add_token(TokenType::RightParam),
    //             '{' => self.add_token(TokenType::LeftBrace),
    //             '}' => self.add_token(TokenType::RightBrace),
    //             ',' => self.add_token(TokenType::Comma),
    //             '.' => self.add_token(TokenType::Dot),
    //             '-' => self.add_token(TokenType::Minus),
    //             '+' => self.add_token(TokenType::Plus),
    //             ';' => self.add_token(TokenType::Semicolon),
    //             '*' => self.add_token(TokenType::Star),

    //             // _ => {

    //             // }
    //         }
    //     }

    //     fn advance(&self) -> u8 {
    //         return self.char_at(self.current += 1);
    //     }

    //     fn add_token(&self, ttype: TokenType) {
    //         self.add_token_n(ttype, None);
    //     }

    //     fn add_token_n(&mut self, ttype: TokenType, literal) {
    //         // let lexeme = self.source[self.start..self.current].to_string();

    //         let lexeme = self.source
    //             .substring(self.start, self.current)
    //             .to_string();
    //         self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    //     }
}
