use regex::Regex;

use crate::utils::should_skip;
use std::fmt::Display;
use std::{fs, io::Error, path::Path, process};

#[derive(Debug)]
pub struct Json {
    path: String,
    blocks: Vec<Block>,
}

impl Json {
    pub fn is_block(line: &str) -> bool {
        !should_skip(line) && line.trim().starts_with("[")
    }

    pub fn from_file<P: AsRef<Path> + Display>(path: P) -> Result<Json, Error> {
        let toml_str = fs::read_to_string(&path)?;
        let lines = toml_str
            .split("\n")
            .into_iter()
            .map(|l| l.trim())
            .collect::<Vec<&str>>();

        let mut idx = 0;
        let mut blocks = vec![];

        while idx < lines.len() {
            let line = lines[idx];
            if should_skip(line) {
                idx += 1;
                continue;
            }

            if Json::is_block(line) {
                blocks.push(Json::parse_block(&lines, idx));
            }

            idx += 1;
        }

        Ok(Json {
            path: path.to_string(),
            blocks,
        })
    }

    pub fn parse_block(lines: &Vec<&str>, idx: usize) -> Block {
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

        let mut end_idx = idx;
        loop {
            end_idx += 1;

            if end_idx >= lines.len() {
                break;
            }

            let line = lines[end_idx];
            if Json::is_block(line) || should_skip(line) {
                break;
            }
        }

        let block_lines = &lines[idx + 1..end_idx];
        Block::new(&block_name, block_lines)
    }
}

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub attrs: Vec<Attr>,
}

impl Block {
    fn new(name: &str, lines: &[&str]) -> Self {
        let attrs = lines.into_iter().map(|&line| Attr::new(line)).collect();

        Block {
            name: name.to_string(),
            attrs,
        }
    }
}

#[derive(Debug)]
pub struct Attr {
    pub key: String,
    pub value: String,
}

impl Attr {
    pub fn new(line: &str) -> Self {
        let re = Regex::new(r#"(?P<key>[^=\n]+)=\s*(?P<value>.*)"#).unwrap();
        let (key, value) = match re.captures(line) {
            Some(cap) => (
                cap["key"].trim().to_string(),
                cap["value"].trim().to_string(),
            ),
            None => {
                eprintln!(r#""{line}" is not a valid TOML"#);
                process::exit(0);
            }
        };

        let re_value = Regex::new(r#"("|'|)(?P<value>[^"|'|\n]+)("|'|)"#).unwrap();
        let value = match re_value.captures(&value) {
            Some(cap) => cap["value"].to_string(),
            None => {
                eprintln!(r#""{}" is not value for the key {}"#, value, key);
                process::exit(0);
            }
        };

        Self { key, value }
    }
}
