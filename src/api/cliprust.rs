use std::{fs, io};
use std::error::Error;
use std::path::Path;
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

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
