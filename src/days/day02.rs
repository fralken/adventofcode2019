use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day02.txt")
        .expect("Something went wrong reading the file");

    let codes = impl_first_star(&contents);

    println!("day  2.1 - value at position 0: {}", codes[0]);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day02.txt")
        .expect("Something went wrong reading the file");

    let (noun, verb) = impl_second_star(&contents);

    println!("day  2.2 - 100 * noun + verb: {}", 100 * noun + verb);
}

fn interpreter(mut codes: Vec<usize>) -> Vec<usize> {
    let mut pos = 0;
    while pos < codes.len() - 4 {
        let res = codes[pos + 3];
        let op1 = codes[pos + 1];
        let op2 = codes[pos + 2];
        match codes[pos] {
            1 => codes[res] = codes[op1] + codes[op2],
            2 => codes[res] = codes[op1] * codes[op2],
            99 => break,
            _ => panic!("wrong opcode {} at position {}", codes[pos], pos)
        }
        pos += 4;
    }
    codes
}

fn extract_codes(contents: &str) -> Vec<usize> {
    contents
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn impl_first_star(contents: &str) -> Vec<usize> {
    let mut codes: Vec<usize> = extract_codes(&contents);

    codes[1] = 12;
    codes[2] = 2;

    interpreter(codes)
}

fn impl_second_star(contents: &str) -> (usize, usize) {
    let input: Vec<usize> = extract_codes(&contents);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes = input.clone();

            codes[1] = noun;
            codes[2] = verb;

            let codes = interpreter(codes);

            if codes[0] == 19_690_720 {
                return (noun, verb)
            }
        }
    }

    panic!("day  2.2 - codes not found for result 19_690_720");
}

#[test]
fn test0_interpreter() {
    assert_eq!(interpreter(extract_codes("1,9,10,3,2,3,11,0,99,30,40,50")), [3500,9,10,70,2,3,11,0,99,30,40,50]);
}

#[test]
fn test1_interpreter() {
    assert_eq!(interpreter(extract_codes("1,0,0,0,99")), [2,0,0,0,99]);
}

#[test]
fn test2_interpreter() {
    assert_eq!(interpreter(extract_codes("2,3,0,3,99")), [2,3,0,6,99]);
}

#[test]
fn test3_interpreter() {
    assert_eq!(interpreter(extract_codes("2,4,4,5,99,0")), [2,4,4,5,99,9801]);
}

#[test]
fn test4_interpreter() {
    assert_eq!(interpreter(extract_codes("1,1,1,4,99,5,6,0,99")), [30,1,1,4,2,5,6,0,99]);
}
