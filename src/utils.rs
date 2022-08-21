pub fn should_skip(line: &str) -> bool {
    line.trim().starts_with("#") || line.trim().len() == 0 
}