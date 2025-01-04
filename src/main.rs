mod lexer;
mod helper_functions;
mod errors;

fn main() -> Result<(), errors::CompilerErrors>
{
    let content = helper_functions::read_file("main.c")?;
    println!("The content of the file is:\n{}", content);
    let lexer_run = lexer::Lexer::new();
    let tokens = lexer_run.get_tokens(&content)?;

    for token in tokens 
    {
        println!("{:?}", token);
    }

    Ok(())
}