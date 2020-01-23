use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day04.txt")
        .expect("Something went wrong reading the file");

    let count = check(&contents, &check1);

    println!("day  4.1 - count of different passwords: {}", count);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day04.txt")
        .expect("Something went wrong reading the file");

    let count = check(&contents, &check2);

    println!("day  4.2 - count of different passwords: {}", count);
}

fn check(contents: &str, check_fun: &dyn Fn(&str) -> bool) -> u32 {
    let limits = contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;
    if limits.len() == 2 {
        for n in limits[0]..=limits[1] {
            if check_fun(&n.to_string()) { count += 1 }
        }
    }
    count
}

fn check1(val: &str) -> bool {
    let chars = val.chars().collect::<Vec<_>>();
    chars.len() == 6 &&
        chars.windows(2).all(|v| v[0] <= v[1]) &&
        chars.windows(2).any(|v| v[0] == v[1])
}

fn check2(val: &str) -> bool {
    let chars = val.chars().collect::<Vec<_>>();
    chars.len() == 6 &&
        chars.windows(2).all(|v| v[0] <= v[1]) &&
        {
            for i in 1..6 {
                if chars[i] == chars[i-1] &&
                    (i == 5 || chars[i] != chars[i+1]) &&
                    (i == 1 || chars[i] != chars[i-2]) {
                    return true
                }
            }
            false
        }
}

#[test]
fn test0_check1() {
    assert_eq!(check1("111111"), true);
}

#[test]
fn test1_check1() {
    assert_eq!(check1("223450"), false);
}

#[test]
fn test2_check1() {
    assert_eq!(check1("123789"), false);
}

#[test]
fn test0_check2() {
    assert_eq!(check2("112233"), true);
}

#[test]
fn test1_check2() {
    assert_eq!(check2("123444"), false);
}

#[test]
fn test2_check2() {
    assert_eq!(check2("111122"), true);
}
