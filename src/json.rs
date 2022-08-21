use regex::Regex;

use crate::utils::should_skip;
use std::process;



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

        let re = Regex::new(r"\[(?P<name>[^\]]+)\]").expect("Invalid regex");
        let block_name = match re.captures(line) {
            Some(cap) => cap["name"].to_string(),
            None => {
                eprintln!("Invalid TOML");
                process::exit(0);
            }
        };

        println!("New block: {}", block_name);


        idx
    }
}