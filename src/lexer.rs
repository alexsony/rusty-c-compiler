use regex::Regex;
use std::{collections::HashMap, fmt::format};
use crate::errors::CompilerErrors;

const KEYWORDS: [&str; 4] = ["main", "void", "int", "return"];
const DELIMITERS: [&str; 5] = ["{", "}", "(", ")", ";"];
const OPERATORS: [&str; 4] = ["+", "-", ":", "/"];

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum TokenType
{
    Keyword,
    Identifier,
    Constant,
    Operator(String),
    Delimiter(String),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token
{
    token_type: TokenType,
    value: String,
    position: usize,
}

pub struct Lexer
{
    patterns: HashMap<TokenType, Regex>
}

impl Lexer 
{
    pub fn new() -> Self 
    {
        Lexer { patterns: Lexer::create_patterns() }
    }

    pub fn get_tokens(&self, input: &str) -> Result<Vec<Token>, CompilerErrors>
    {
        let mut tokens:Vec<Token> = Vec::new();
        let mut unknown_tokens:Vec<Token> = Vec::new();
        let mut position = 0;
    
        while position < input.len()
        {
            let mut matched = false;
            
            for (token_type, regex) in &self.patterns
            {
                match regex.find(&input[position..])
                {
                    Some(match_value) if match_value.start() == 0 =>
                    {
                        tokens.push(Token {
                            token_type: Lexer::is_keyword(match_value.as_str().to_string(), token_type.clone()),
                            value: match_value.as_str().to_string(),
                            position: position,
                        });
                        position += match_value.end();
                        matched = true;
                        break;
                    }
                    _ => continue
                }
            }
    
            if !matched
            {
                match input.chars().nth(position)
                {
                    Some(c) if c.is_whitespace() => position += 1,
                    Some(c) => {
                        unknown_tokens.push(Token {
                            token_type: TokenType::Unknown,
                            value: c.to_string(),
                            position: position,
                        });
                        position += 1;
                    }
    
                    None => break,
                }
            }
        }

        self.validate_tokens(&unknown_tokens)?;
        Ok(tokens)
    }

    fn create_patterns() -> HashMap<TokenType, Regex>
    {
        let mut regexes: HashMap<TokenType, Regex> = HashMap::new();
        let mut patterns_to_add: Vec<(TokenType, String)> = Vec::new();

        patterns_to_add.push((TokenType::Constant, r"\b\d+\b".into()));
        patterns_to_add.push((TokenType::Identifier, r"[a-zA-Z_]\w*".into()));

        for operator in OPERATORS
        {
            let regex_value = format!(r"{}", regex::escape(operator));
            patterns_to_add.push((TokenType::Operator(operator.to_string()), regex_value));
        }

        for delimiter in DELIMITERS
        {
            let regex_value = format!(r"{}", regex::escape(delimiter));
            patterns_to_add.push((TokenType::Delimiter(delimiter.to_string()), regex_value));
        }

        for (token_type, pattern) in patterns_to_add
        {
            match Regex::new(&pattern)
            {
                Ok(regex) => { regexes.insert(token_type, regex); },
                Err(err) => { 
                    eprintln!("Error compiling the regex for {} : {}", pattern, err);
                },
            };
        }
        regexes
    }

    fn validate_tokens(&self, unknown_tokens: &Vec<Token>) -> Result<(), CompilerErrors>
    {
        if !unknown_tokens.is_empty()
        {
            for token in unknown_tokens
            {
                println!("{:?}", token);
            }
            return Err(CompilerErrors::LexerError("Wrong character used!".into()));
        }

        Ok(())
    }

    fn is_keyword(found_value: String, found_token_type: TokenType) -> TokenType
    {
        if found_token_type != TokenType::Identifier 
        {
            return found_token_type;
        }

        let check_value = found_value.trim_end_matches(r"\b");
        if KEYWORDS.contains(&check_value)
        {
            return TokenType::Keyword;
        }

        return TokenType::Identifier;
    }
}

#[cfg(test)]
mod tests
{
    use crate::helper_functions;

    use super::*;
    use lazy_static::lazy_static;

    lazy_static! 
    {
        static ref LEXER: Lexer = Lexer::new();
    }

    #[test]
    fn test_valid_files() -> Result<(), CompilerErrors>
    {
        let files = helper_functions::read_files("test_files/lexer/valid")?;

        for file in files
        {
            let tokens = LEXER.get_tokens(&file).unwrap();
            assert!(!tokens.is_empty());
        }
        Ok(())
    }
}
