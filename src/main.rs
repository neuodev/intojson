mod json;
mod utils;

use std::io::Error;

use json::Json;

fn main() -> Result<(), Error> {
    let json = Json::from_file("example.toml")?;
    let result = json.save()?;

    println!("{:#?}", result);
    Ok(())
}
