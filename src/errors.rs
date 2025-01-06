use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CompilerErrors
{
    IOError(String),
    LexerError(String)
}

impl fmt::Display for CompilerErrors
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            CompilerErrors::IOError(ref msg) => write!(f, "I/O Error: {}", msg),
            CompilerErrors::LexerError(ref msg) => write!(f, "Lexer error: {}", msg)
        }
    }
}

impl From<io::Error> for CompilerErrors 
{
    fn from(err: io::Error) -> CompilerErrors 
    {
        CompilerErrors::IOError(err.to_string())
    }
}

impl From<String> for CompilerErrors 
{
    fn from(err: String) -> CompilerErrors 
    {
        CompilerErrors::LexerError(err)
    }
}