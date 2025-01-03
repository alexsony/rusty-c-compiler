use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum LexerError {
    InvalidInput(String),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum TokenType
{
    Identifier,
    Constant,
    Keyword(String),
    Symbol(String),
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

    pub fn get_tokens(&self, input: &str) -> Result<Vec<Token>, LexerError>
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
                            token_type: token_type.clone(),
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
        
        let patterns_to_add = vec![
            (TokenType::Identifier, r"[a-zA-Z_]\w*"),
            (TokenType::Constant, r"\b\d+\b"),
            (TokenType::Keyword("int".to_string()), r"int\b"),
            (TokenType::Keyword("void".to_string()), r"void\b"),
            (TokenType::Keyword("return".to_string()), r"return\b"),
            (TokenType::Symbol("(".to_string()), r"\("),
            (TokenType::Symbol(")".to_string()), r"\)"),
            (TokenType::Symbol("{".to_string()), r"\{"),
            (TokenType::Symbol("}".to_string()), r"\}"),
            (TokenType::Symbol(";".to_string()), r";"),
        ];

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

    fn validate_tokens(&self, unknown_tokens: &Vec<Token>) -> Result<(), LexerError>
    {
        if !unknown_tokens.is_empty()
        {
            for token in unknown_tokens
            {
                println!("{:?}", token);
            }
            return Err(LexerError::InvalidInput("Wrong character used!".into()));
        }

        Ok(())
    }
}
