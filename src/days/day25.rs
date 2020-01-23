use std::fs;
use std::io;
use crate::intcode::{ IntCode, extract_codes };

pub fn last_star() {
    let contents = fs::read_to_string("./input/day25.txt")
        .expect("Something went wrong reading the file");

    let mut droid = IntCode::new(extract_codes(&contents));
    while !droid.interpreter() {
        println!("{}", droid.read_string());
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        droid.write_string(&line);
    }

    println!("{} - THE END -", droid.read_string());
}
