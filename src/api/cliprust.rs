use std::fs;
use std::error::Error;
pub fn copy(source: &str, destination: &str) -> Result<(), Box<dyn Error>>{
    fs::copy(&source, &destination)?;
    println!("Successfully copied from {} to {}", source, destination);
    Ok(())
}

pub fn cut(source: &str, destination: &str) -> Result<(), Box<dyn Error>>{
        fs::copy(&source, &destination)?;
        fs::remove_file(&source)?;
        println!("Successfully copied from {} to {}, and deleted {}", source, destination, source);
        Ok(())
}
