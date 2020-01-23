use std::fs;
use permutohedron::Heap;

#[derive(Default)]
struct Ampli {
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    codes: Vec<i32>,
    pos: usize
}

impl Ampli {
    fn new(codes: Vec<i32>, inputs: Vec<i32>) -> Self {
        Ampli { inputs, codes, ..Default::default() }
    }

    fn get_param_value(&self, mode: i32, param: usize) -> i32 {
        let param_mode = (mode / 10_i32.pow(param as u32 + 1)) % 10;
        match param_mode {
            0 => {
                let op = self.codes[self.pos + param] as usize;
                self.codes[op]
            },
            1 => self.codes[self.pos + param],
            _ => panic!("wrong parameter mode {} at position {} for opcode {}", param_mode, self.pos, mode)
        }
    }

    fn interpreter(&mut self) -> usize {
        while self.pos < self.codes.len() {
            let mode = self.codes[self.pos];
            let opcode = mode % 100;
            match opcode {
                1 => {
                    let res = self.codes[self.pos + 3] as usize;
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    self.codes[res] = val1 + val2;
                    self.pos += 4;
                },
                2 => {
                    let res = self.codes[self.pos + 3] as usize;
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    self.codes[res] = val1 * val2;
                    self.pos += 4;
                },
                3 => {
                    let res = self.codes[self.pos + 1] as usize;
                    if self.inputs.is_empty() {
                        return 0;
                    } else {
                        self.codes[res] = self.inputs.remove(0);
                        self.pos += 2;
                    }
                },
                4 => {
                    self.outputs.push(self.get_param_value(mode, 1));
                    self.pos += 2;
                },
                5 => {
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    if val1 != 0 { self.pos = val2 as usize } else { self.pos += 3; };
                },
                6 => {
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    if val1 == 0 { self.pos = val2 as usize } else { self.pos += 3; };
                },
                7 => {
                    let res = self.codes[self.pos + 3] as usize;
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    self.codes[res] = if val1 < val2 { 1 } else { 0 };
                    self.pos += 4;
                },
                8 => {
                    let res = self.codes[self.pos + 3] as usize;
                    let val1 = self.get_param_value(mode, 1);
                    let val2 = self.get_param_value(mode, 2);
                    self.codes[res] = if val1 == val2 { 1 } else { 0 };
                    self.pos += 4;
                },
                99 => break,
                _ => panic!("wrong opcode {} at position {}", self.codes[self.pos], self.pos)
            }
        }
        1
    }
}

pub fn first_star() {
    let contents = fs::read_to_string("./input/day07.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut signal = 0;
    let mut data = [0, 1, 2, 3, 4];
    let mut permutations = Heap::new(&mut data);
    while let Some(values) = permutations.next_permutation() {
        signal = signal.max(impl_first_star(&codes, values));
    }

    println!("day  7.1 - highest signal that can be sent to the thrusters: {}", signal);
}

pub fn second_star() {
    let contents = fs::read_to_string("./input/day07.txt")
        .expect("Something went wrong reading the file");

    let codes = extract_codes(&contents);
    let mut signal = 0;
    let mut data = [5, 6, 7, 8, 9];
    let mut permutations = Heap::new(&mut data);
    while let Some(values) = permutations.next_permutation() {
        signal = signal.max(impl_second_star(&codes, values));
    }

    println!("day  7.2 - highest signal that can be sent to the thrusters: {}", signal);
}

fn impl_first_star(codes: &[i32], inputs: &[i32]) -> i32 {
    let mut output = 0;
    for input in inputs {
        let mut ampli = Ampli::new(codes.to_owned(), vec![*input, output]);
        let _ = ampli.interpreter();
        output = *ampli.outputs.get(0).unwrap();
    }
    output
}

fn impl_second_star(codes: &[i32], inputs: &[i32]) -> i32 {
    let mut amplis = inputs
        .iter()
        .map(|v| Ampli::new(codes.to_owned(), vec![*v]))
        .collect::<Vec<Ampli>>();
    let mut last_output = 0;
    let mut completed = 0;
    let mut output = vec![0];
    while completed < amplis.len() {
        for ampli in amplis.iter_mut() {
            ampli.inputs.append(&mut output);
            completed += ampli.interpreter();
            output = ampli.outputs.drain(..).collect();
        };
        last_output = *output.get(0).unwrap();
    }
    last_output
}

fn extract_codes(contents: &str) -> Vec<i32> {
    contents
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test0_first_star() {
    assert_eq!(impl_first_star(&extract_codes("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), &[4,3,2,1,0]), 43210);
}

#[test]
fn test1_first_star() {
    assert_eq!(impl_first_star(&extract_codes("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), &[0,1,2,3,4]), 54321);
}

#[test]
fn test2_first_star() {
    assert_eq!(impl_first_star(&extract_codes("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), &[1,0,4,3,2]), 65210);
}

#[test]
fn test0_second_star() {
    assert_eq!(impl_second_star(&extract_codes("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), &[9,8,7,6,5]), 139629729);
}

#[test]
fn test1_second_star() {
    assert_eq!(impl_second_star(&extract_codes("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), &[9,7,8,5,6]), 18216);
}
