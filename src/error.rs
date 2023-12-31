#![allow(unused)]
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::scanner::token::Token;

#[derive(Error, Diagnostic, Debug)]
pub enum LoxError {
    /* 
    The #[error(transparent)] attribute on the first variant `IOError` indicates that this variant should transparently wrap the `std::io::Error` type. 
    This means that if an `std::io::Error` is encountered, it will be automatically converted to a `LoxError::IOError` variant.
    */

    /*
        The #[diagnostic] attribute is used to define a diagnostic code for the error. 
    */
    #[error(transparent)]
    #[diagnostic(
        code(lox_error::IO_error),
        // url(docsrs),
        // help("try doing it better next time?")
    )]
    IOError(#[from] std::io::Error),

    /*
        variant is used to represent errors that occur during the parsing of a Lox program
    */
    /*
    The #[error()] attribute is used to define the error message for an enum variant in Rust
    */
    #[error("compilation failed because of syntax errors")]
    #[diagnostic(code(lox_error::Syntax_error))]
    SyntaxErrors(#[related] Vec<SyntaxError>),



    /// The {err_message} and {line} placeholders in the error message will be replaced with 
    /// the values of the err_message and line fields, respectively, when the error is formatted.
    #[diagnostic(code(lox_error::arithmetic_error))]
    #[error("arithmetic error: {err_message} at line {line}")]
    RuntimeError {
        line: usize,
        #[label("here")]
        span: SourceSpan,
        err_message: String,
        #[help]
        help_message: String,
    },
}

impl LoxError {
    pub(crate) fn new_runtime_err(token: &Token, err_msg: String, help_msg: String) -> LoxError {
        LoxError::RuntimeError {
            line: token.line,
            span: token.span.into(),
            err_message: err_msg,
            help_message: help_msg,
        }
    }
}




#[derive(Error, Diagnostic, Debug)]
#[diagnostic(code(lox_error::syntax_error))]
#[error("syntax error: {err_message} at line {line}")]
pub struct SyntaxError {
    pub line: usize,
    #[label("here")]
    pub span: SourceSpan,
    pub err_message: String,
    #[help]
    pub help_message: String,
}

impl SyntaxError {
    pub(crate) fn new(token: &Token, err: String, help: String) -> Self {
        SyntaxError {
            line: token.line,
            span: token.span.into(),
            err_message: err,
            help_message: help,
        }
    }
}

