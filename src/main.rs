use std::fs::File;
use std::io::{self, Read};

mod lexer;

// TODO: change the error handling 
fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap(); 
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();  

    content
}

fn main() -> Result<(), lexer::LexerError>
{
    let content = read_file("main.c");
    println!("The content of the file is:\n{}", content);
    let lexer_run = lexer::Lexer::new();
    let tokens = lexer_run.get_tokens(&content)?;

    for token in tokens 
    {
        println!("{:?}", token);
    }

    Ok(())
}