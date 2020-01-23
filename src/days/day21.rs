use std::fs;
use crate::intcode::{ IntCode, extract_codes };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day21.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut jump_droid = IntCode::new(codes);
    let instructions = "\
            OR A T\n\
            AND B T\n\
            AND C T\n\
            NOT T J\n\
            AND D J\n\
            WALK\n\
        ";

    jump_droid.write_string(instructions);
    jump_droid.interpreter();
    let output = jump_droid.read();
    let result = output.last().unwrap();
    println!("day 21.1 - amount of reported damage to the hull: {}", result);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day21.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut jump_droid = IntCode::new(codes);
    let instructions = "\
            OR A T\n\
            AND B T\n\
            AND C T\n\
            NOT T J\n\
            OR E T\n\
            OR H T\n\
            AND T J\n\
            AND D J\n\
            RUN\n\
        ";

    jump_droid.write_string(instructions);
    jump_droid.interpreter();
    let output = jump_droid.read();
    let result = output.last().unwrap();
    println!("day 21.2 - amount of reported damage to the hull: {}", result);
}
