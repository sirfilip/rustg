use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Missing filename argument");
        },
        2 => {
            println!("Missing regex");
        },
        3 => {
            let re = Regex::new(&args[1]).unwrap();
            let path = Path::new(&args[2]);
            
            let file = match File::open(&path) {
                Err(why) => panic!("could not open {}: {}", path.display(), why),
                Ok(file) => file,
            };

            let reader = BufReader::new(file);

            for (index, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                if re.is_match(&line) {
                    println!("{}: {}", index+1, line);
                }
            }
        }
        _ => {
            println!("Usage....");
        }
    }
}
