use std::{fs, io::Error};

fn main() -> Result<(), Error>{
    let toml_str = fs::read_to_string("example.toml")?;
    let lines = toml_str.split("\n");

    for line in lines {
        println!("> {}", line);
    }

    Ok(())
}
