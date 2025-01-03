use std::fs::File;
use std::io::{self, Read};

use regex::Regex;
use std::collections::HashMap;


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
struct Token
{
    token_type: TokenType,
    value: String,
    position: usize,
}

fn read_file(file_path: &str) -> std::io::Result<String>
{
    let mut file = File::open(file_path)?; 
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
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

fn lexer(input: &str) -> Vec<Token>
{
    let patterns = create_patterns();
    let mut tokens:Vec<Token> = Vec::new();
    let mut position = 0;

    while position < input.len()
    {
        let mut matched = false;
        
        for (token_type, regex) in &patterns
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
                    tokens.push(Token {
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

    tokens
}
fn main() -> io::Result<()>
{
    let content = read_file("main.c")?;
    println!("The content of the file is:\n{}", content);
    let tokens = lexer(&content);

    for token in tokens 
    {
        println!("{:?}", token);
    }

    Ok(())
}