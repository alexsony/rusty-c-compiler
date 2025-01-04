use std::fs::File;
use std::io::Read;

use crate::errors::CompilerErrors;

pub fn read_file(file_path: &str) -> Result<String, CompilerErrors> 
{
    let mut file = File::open(file_path).map_err(CompilerErrors::from)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(CompilerErrors::from)?;

    Ok(content)
}