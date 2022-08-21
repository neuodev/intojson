mod utils;
mod json;

use std::{fs, io::Error};
use utils::should_skip;

use crate::json::Json;

fn main() -> Result<(), Error> {
    let toml_str = fs::read_to_string("example.toml")?;
    let lines = toml_str.split("\n").into_iter().map(|l| l.trim()).collect::<Vec<&str>>();

    let mut idx = 0;

    while idx < lines.len() {
        let line = lines[idx];
        if should_skip(line) {
            idx += 1;
            continue; 
        }

        // println!("> {}", line);

        if Json::is_block(line) {
            Json::from_block(&lines, idx);
        }

        idx+=1;
    }

    Ok(())
}

// Comments 
// root key value 
// sections