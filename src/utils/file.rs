use std::error::Error;
use std::fs;

#[allow(dead_code)]
fn read_file(path: &'static str) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let bys = fs::read(path)?;

    Ok((content, bys))
}
