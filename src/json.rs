use std::process;

use crate::utils::should_skip;

#[derive(Debug)]
pub struct Json;

impl Json {
    pub fn is_block(line: &str) -> bool {
        !should_skip(line) && line.trim().starts_with("[")
    }


    pub fn from_block(lines: &Vec<&str>, idx: usize) -> usize {
        let line = lines[idx];
        
        if !Json::is_block(line) {
            eprint!("Invalid TOML block");
            process::exit(0);
        }


        println!("block title: {}", line);

        idx
    }
}