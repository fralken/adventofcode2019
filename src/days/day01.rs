use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day01.txt")
        .expect("Something went wrong reading the file");

    let fuel = impl_first_star(&contents);

    println!("day  1.1 - sum of the fuel requirements: {}", fuel);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day01.txt")
        .expect("Something went wrong reading the file");

    let fuel = impl_second_star(&contents);

    println!("day  1.2 - sum of the fuel requirements: {}", fuel);
}

fn impl_first_star(contents: &str) -> u32 {
    contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap() / 3 - 2)
        .sum()
}

fn impl_second_star(contents: &str) -> u32 {
    contents
        .lines()
        .map(|s| {
            let mut mass = s.parse::<i32>().unwrap();
            let mut fuel = 0;
            while mass > 0 {
                mass = mass / 3 - 2;
                if mass > 0 { fuel += mass };
            }
            fuel as u32
        })
        .sum()
}

#[test]
fn test0_first_star() {
    assert_eq!(impl_first_star("12"), 2);
}

#[test]
fn test1_first_star() {
    assert_eq!(impl_first_star("14"), 2);
}

#[test]
fn test2_first_star() {
    assert_eq!(impl_first_star("1969"), 654);
}

#[test]
fn test3_first_star() {
    assert_eq!(impl_first_star("100756"), 33583);
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star("14"), 2);
}

#[test]
fn test1_second_star() {
    assert_eq!(impl_second_star("1969"), 966);
}

#[test]
fn test2_second_star() {
    assert_eq!(impl_second_star("100756"), 50346);
}
