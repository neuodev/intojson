use regex::Regex;
use std::process;

#[derive(Debug)]
pub enum ValueType {
    Object,
    Number,
    Array,
    String,
    Boolean,
}
pub fn should_skip(line: &str) -> bool {
    line.trim().starts_with("#") || line.trim().len() == 0
}

pub fn get_type(entry: &str) -> ValueType {
    if entry == "true" || entry == "false" {
        return ValueType::Boolean;
    }

    if entry.parse::<f64>().is_ok() {
        return ValueType::Number;
    }

    let types_set = [
        (ValueType::Array, r#"\[.*\]"#),
        (ValueType::Object, r#"\{.*\}"#),
        (ValueType::String, r#"("|'|)(?P<value>[^"|'|\n]+)("|'|)"#),
    ];

    for (value_type, re) in types_set {
        let re = Regex::new(re).unwrap();

        if re.captures(entry).is_some() {
            return value_type;
        }
    }

    eprintln!("Unspported type of the right hand type");
    process::exit(0);
}

pub fn get_string(entry: &str) {
    let re = Regex::new(r#"("|'|)(?P<value>[^"|'|\n]+)("|'|)"#).unwrap();
    let value = match re.captures(&entry) {
        Some(cap) => cap["value"].to_string(),
        None => {
            eprintln!(r#""{}" is not value for the key {}"#, value, key);
            process::exit(0);
        }
    };
}
