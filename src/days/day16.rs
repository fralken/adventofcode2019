use std::fs;
use std::iter::repeat;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day16.txt")
        .expect("Something went wrong reading the file");

    let digits = impl_first_star(extract_digits(&contents), 100);
    println!("day 16.1 - first eight digits in the final output list: {}",
             digits.iter().fold(String::new(), |a, d| format!("{}{}", a, d)));
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day16.txt")
        .expect("Something went wrong reading the file");

    let digits = impl_second_star(extract_digits(&contents), 10_000, 100);
    println!("day 16.2 - eight-digit message embedded in the final output list: {}",
             digits.iter().fold(String::new(), |a, d| format!("{}{}", a, d)));
}

fn impl_first_star(input: Vec<i8>, phases: usize) -> Vec<i8> {
    let mut digits = input;
    for _ in 0..phases {
        digits = next_phase(&digits);
    }
    digits[..8].to_vec()
}

fn impl_second_star(input: Vec<i8>, repeat: usize, phases: usize) -> Vec<i8> {
    let mut digits = input.repeat(repeat);
    let offset = input[..7].iter().fold(0, |a, d| a * 10 + *d as i32) as usize;
    if offset > digits.len() / 2 {
        // digits in the second half of the array are computed as the the sum of the following computed digits
        // computation is easily done in reverse starting from the end
        // we can stop computation at offset because we don't need values before offset
        for _ in 0..phases {
            for i in (offset..digits.len()-1).rev() {
                digits[i] = (digits[i] + digits[i+1]) % 10;
            }
        }
    } else {
        for _ in 0..phases {
            digits = next_phase(&digits);
        }
    }
    digits[offset..offset+8].to_vec()
}

fn pattern(i: usize) -> impl Iterator<Item = i8> {
    repeat(0).take(i)
        .chain(repeat(1).take(i))
        .chain(repeat(0).take(i))
        .chain(repeat(-1).take(i))
        .cycle()
        .skip(1)
}

fn next_phase(digits: &[i8]) -> Vec<i8> {
    let mut next_phase = Vec::new();
    for i in 0..digits.len() {
        let next_digit = digits
            .iter()
            .zip(pattern(i + 1))
            .filter(|(_, p)| *p != 0)
            .map(|(&d, p)| (d * p) as i32)
            .sum::<i32>()
            .abs() % 10;
        next_phase.push(next_digit as i8);
    }
    next_phase
}

fn extract_digits(contents: &str) -> Vec<i8> {
    contents.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
}

#[test]
fn test0_first_star() {
    assert_eq!(impl_first_star(extract_digits("12345678"), 4), [0,1,0,2,9,4,9,8])
}

#[test]
fn test1_first_star() {
    assert_eq!(impl_first_star(extract_digits("80871224585914546619083218645595"), 100), [2,4,1,7,6,1,7,6])
}

#[test]
fn test2_first_star() {
    assert_eq!(impl_first_star(extract_digits("19617804207202209144916044189917"), 100), [7,3,7,4,5,4,1,8])
}

#[test]
fn test3_first_star() {
    assert_eq!(impl_first_star(extract_digits("69317163492948606335995924319873"), 100), [5,2,4,3,2,1,3,3])
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star(extract_digits("03036732577212944063491565474664"), 10_000, 100), [8,4,4,6,2,0,2,6])
}

#[test]
fn test1_second_star() {
    assert_eq!(impl_second_star(extract_digits("02935109699940807407585447034323"), 10_000, 100), [7,8,7,2,5,2,7,0])
}

#[test]
fn test2_second_star() {
    assert_eq!(impl_second_star(extract_digits("03081770884921959731165446850517"), 10_000, 100), [5,3,5,5,3,7,3,1])
}