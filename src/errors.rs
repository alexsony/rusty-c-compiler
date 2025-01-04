use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CompilerErrors
{
    IOError(io::Error),
    LexerError(String)
}

impl fmt::Display for CompilerErrors
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            CompilerErrors::IOError(ref err) => write!(f, "I/O Error: {}", err),
            CompilerErrors::LexerError(ref msg) => write!(f, "Lexer error: {}", msg)
        }
    }
}

impl From<io::Error> for CompilerErrors 
{
    fn from(err: io::Error) -> CompilerErrors 
    {
        CompilerErrors::IOError(err)
    }
}

impl From<String> for CompilerErrors 
{
    fn from(err: String) -> CompilerErrors 
    {
        CompilerErrors::LexerError(err)
    }
}