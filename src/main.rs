use std::fs::File;
use std::io::{BufRead, BufReader};

use dir;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vimrc = dir::home_dir().unwrap();
    vimrc.push(".vimrc");

    let mut count = 0;
    let regex = Regex::new("^\\s*$|^\\s*\"|^\\s*\\\\").unwrap();
    for line in BufReader::new(File::open(vimrc)?).lines() {
        let l = line?;
        if regex.is_match(&l) {
            continue;
        }
        count += 1;
    }

    println!("line count: {}", count);

    Ok(())
}
