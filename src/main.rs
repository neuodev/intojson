mod json;
mod utils;

use std::io::Error;
use std::path::Path;
use std::{env, process};
use json::Json;
use std::thread;

fn main() -> Result<(), Error> {
    let files = env::args().skip(1).collect::<Vec<String>>();

    if files.len() == 0 {
        eprintln!("No files provided");
        process::exit(0);
    }

    files.into_iter().map(|f| {
        thread::spawn(|| {
            let json = Json::from_file(f).unwrap();
            json.save().expect("Unable to save json file");
        })
    }).for_each(|h| h.join().unwrap());

    Ok(())
}
