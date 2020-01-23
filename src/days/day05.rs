use std::fs;

pub fn first_star() {
    let contents = fs::read_to_string("./input/day05.txt")
        .expect("Something went wrong reading the file");

    let output = interpreter(extract_codes(&contents), 1).1;

    println!("day  5.1 - diagnostic code for input 1: {}", output);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day05.txt")
        .expect("Something went wrong reading the file");

    let output = interpreter(extract_codes(&contents), 5).1;

    println!("day  5.2 - diagnostic code for system ID 5: {}", output);
}

fn interpreter(mut codes: Vec<i32>, input: i32) -> (Vec<i32>, i32) {
    fn get_param_value(mode: i32, param: usize, codes: &[i32], pos: usize) -> i32 {
        let param_mode = (mode / 10_i32.pow(param as u32 + 1)) % 10;
        match param_mode {
            0 => {
                let op = codes[pos + param] as usize;
                codes[op]
            },
            1 => codes[pos + param],
            _ => panic!("wrong parameter mode {} at position {} for opcode {}", param_mode, pos, mode)
        }
    };

    let mut output = 0;
    let mut pos = 0;

    while pos < codes.len() {
        let mode = codes[pos];
        let opcode = mode % 100;
        match opcode {
            1 => {
                let res = codes[pos + 3] as usize;
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                codes[res] = val1 + val2;
                pos += 4;
            },
            2 => {
                let res = codes[pos + 3] as usize;
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                codes[res] = val1 * val2;
                pos += 4;
            },
            3 => {
                let res = codes[pos + 1] as usize;
                codes[res] = input;
                pos += 2;
            },
            4 => {
                output = get_param_value(mode, 1, &codes, pos);
                pos += 2;
            },
            5 => {
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                if val1 != 0 { pos = val2 as usize } else { pos += 3; };
            },
            6 => {
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                if val1 == 0 { pos = val2 as usize } else { pos += 3; };
            },
            7 => {
                let res = codes[pos + 3] as usize;
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                codes[res] = if val1 < val2 { 1 } else { 0 };
                pos += 4;
            },
            8 => {
                let res = codes[pos + 3] as usize;
                let val1 = get_param_value(mode, 1, &codes, pos);
                let val2 = get_param_value(mode, 2, &codes, pos);
                codes[res] = if val1 == val2 { 1 } else { 0 };
                pos += 4;
            },
            99 => break,
            _ => panic!("wrong opcode {} at position {}", codes[pos], pos)
        }
    }
    (codes, output)
}

fn extract_codes(contents: &str) -> Vec<i32> {
    contents
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test0_interpreter() {
    assert_eq!(interpreter(extract_codes("1002,4,3,4,33"), 0).0, [1002,4,3,4,99]);
}

#[test]
fn test1_interpreter() {
    for i in 0..=10 {
        if i == 8 {
            assert_eq!(interpreter(extract_codes("3,9,8,9,10,9,4,9,99,-1,8"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,9,8,9,10,9,4,9,99,-1,8"), i).1, 0);
        }
    }
}

#[test]
fn test2_interpreter() {
    for i in 0..=10 {
        if i < 8 {
            assert_eq!(interpreter(extract_codes("3,9,7,9,10,9,4,9,99,-1,8"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,9,7,9,10,9,4,9,99,-1,8"), i).1, 0);
        }
    }
}

#[test]
fn test3_interpreter() {
    for i in 0..=10 {
        if i == 8 {
            assert_eq!(interpreter(extract_codes("3,3,1108,-1,8,3,4,3,99"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,3,1108,-1,8,3,4,3,99"), i).1, 0);
        }
    }
}

#[test]
fn test4_interpreter() {
    for i in 0..=10 {
        if i < 8 {
            assert_eq!(interpreter(extract_codes("3,3,1107,-1,8,3,4,3,99"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,3,1107,-1,8,3,4,3,99"), i).1, 0);
        }
    }
}

#[test]
fn test5_interpreter() {
    for i in -5..=5 {
        if i != 0 {
            assert_eq!(interpreter(extract_codes("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), i).1, 0);
        }
    }
}

#[test]
fn test6_interpreter() {
    for i in -5..=5 {
        if i != 0 {
            assert_eq!(interpreter(extract_codes("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), i).1, 1);
        } else {
            assert_eq!(interpreter(extract_codes("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), i).1, 0);
        }
    }
}

#[test]
fn test7_interpreter() {
    for i in 0..=10 {
        if i < 8 {
            assert_eq!(interpreter(extract_codes("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), i).1, 999);
        } else if i > 8{
            assert_eq!(interpreter(extract_codes("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), i).1, 1001);
        } else {
            assert_eq!(interpreter(extract_codes("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), i).1, 1000);
        }
    }
}
