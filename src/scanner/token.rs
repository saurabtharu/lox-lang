use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Assign, // ('=')
    Equals, // ('==')
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug)]
pub(crate) enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "flase"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
    pub span: (usize, usize),
}

impl Token {
    // TODO: implement it as span: (self.start, self.current)
    pub(crate) fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
        span: (usize, usize),
    ) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
            span,
        }
    }

    // pub(crate) fn eof(line: usize) -> Token {
    //     Token {
    //         ttype: TokenType::EOF,
    //         lexeme: String::from(""),
    //         literal: None,
    //         line,
    //     }
    // }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {} @ {}:{}] {}: {:?} {}",
            self.line,
            self.span.0,
            self.span.1,
            self.lexeme,
            self.ttype,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     write!(
    //         f,
    //         "{:#?} {} {}",
    //         self.ttype,
    //         self.lexeme,
    //         if let Some(literal) = &self.literal {
    //             literal.to_string()
    //         } else {
    //             "None".to_string()
    //         }
    //     )
    // }
}
