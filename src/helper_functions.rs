use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::errors::CompilerErrors;

pub fn read_file(file_path: &str) -> Result<String, CompilerErrors> 
{
    let mut file = File::open(file_path).map_err(CompilerErrors::from)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(CompilerErrors::from)?;

    Ok(content)
}

pub fn read_files(dir_path: &str) -> Result<Vec<String>, CompilerErrors>
{
    let directory = Path::new(dir_path);
    let mut files: Vec<String> = Vec::new();

    if !directory.is_dir()
    {
        return Err(CompilerErrors::IOError(
            format!("{} is not a directory", dir_path).into())
        );
    }

    for entry in fs::read_dir(directory)?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
        {
            println!("Reading file {:?}", path);
            let content = fs::read_to_string(&path)?;
            files.push(content);
        }
    }

    Ok(files)
}