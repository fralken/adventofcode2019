use std::fs;
use crate::intcode::{ IntCode, extract_codes };

pub fn first_star() {
    let contents = fs::read_to_string("./input/day09.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let output = compute(&codes, &[1]);

    println!("day  9.1 - BOOST keycode: {:?}", output[0]);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day09.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let output = compute(&codes, &[2]);

    println!("day  9.2 - coordinates of the distress signal: {:?}", output[0]);
}

fn compute(codes: &[i64], input: &[i64]) -> Vec<i64> {
    let mut ampli = IntCode::new(codes.to_owned());
    ampli.write(input);
    ampli.process();
    ampli.read()
}

#[test]
fn test0_compute() {
    assert_eq!(compute(&extract_codes("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), &[]), [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
}

#[test]
fn test1_compute() {
    assert_eq!(compute(&extract_codes("1102,34915192,34915192,7,4,7,99,0"), &[]), [1219070632396864]);
}

#[test]
fn test2_compute() {
    assert_eq!(compute(&extract_codes("104,1125899906842624,99"), &[]), [1125899906842624]);
}
