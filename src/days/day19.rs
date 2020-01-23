use std::fs;
use crate::intcode::{ IntCode, extract_codes };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day19.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut total = 0;
    for x in 0..50 {
        for y in 0..50 {
            total += run_droid(x, y, &codes);
        }
    }

    println!("day 19.1 - points affected by the tractor beam in the 50x50 area closest to the emitter: {}", total);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day19.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut x = 0;
    let mut y = 0;
    let mut found = false;

    while !found {
        while run_droid(x, y, &codes) == 0 {
            x += 1;
        }
        let start_x = x;
        while run_droid(x + 99, y, &codes) == 1 {
            if run_droid(x, y + 99, &codes) == 1 && run_droid(x + 99, y + 99, &codes) == 1 {
                found = true;
                break;
            }
            x += 1;
            if run_droid(x, y, &codes) == 0 {
                x = start_x;
                break;
            }
        }
        if !found { y += 1; }
    }

    println!("day 19.2 - coordinates of point closest to the emitter of the 100x100 square (10000 * x + y): {}", 10_000 * x + y);
}

fn run_droid(x: i64, y: i64, codes: &[i64]) -> i64 {
    let mut droid = IntCode::new(codes.to_owned());
    droid.write(&[x, y]);
    droid.interpreter();
    droid.read().pop().unwrap()
}