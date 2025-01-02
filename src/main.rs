use std::fs::File;
use std::io::{self, Read};

use regex::Regex;
use std::collections::HashMap;

fn read_file(file_path: &str) -> std::io::Result<String>
{
    let mut file = File::open(file_path)?; 
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn lexer()
{
    let patterns: HashMap<&str, &str> = [
        ("Identifier", r"[a-zA-Z_]\w*\b"),
        ("Constant", r"[0-9]+\b"),
        ("int keyword", r"int\b"),
        ("void keyword", r"void\b"),
        ("return keyword", r"return\b"),
        ("Open parenthesis", r"\("),
        ("Close parenthesis", r"\)"),
        ("Open brace", r"\{"),
        ("Close brace", r"\}"),
        ("Semicolon", r";"),
    ].iter().cloned().collect();

    let mut regexes: HashMap<&str, Regex> = HashMap::new();
}
fn main() -> io::Result<()>
{
    let content = read_file("main.c")?;
    println!("The content of the file is:\n{}", content);

    Ok(())
}