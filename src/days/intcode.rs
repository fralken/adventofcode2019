//
// This IntCode is the final evolution of days 2, 5, 7, 9
//
// It is used in days 9, 11, 13, 15, 17, 19, 21, 23, 25
//

#[derive(Default)]
pub struct IntCode {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    codes: Vec<i64>,
    pos: usize,
    base: usize
}

impl IntCode {
    pub fn new(codes: Vec<i64>) -> Self {
        IntCode { codes, ..Default::default() }
    }

    pub fn init_code(&mut self, d: i64) {
        self.codes[0] = d;
    }

    pub fn read(&mut self) -> Vec<i64> {
        self.outputs.drain(..).collect()
    }

    pub fn read_string(&mut self) -> String {
        self.outputs.drain(..).map(|d| d as u8 as char).collect()
    }

    pub fn no_output(&self) -> bool {
        self.outputs.is_empty()
    }

    pub fn write(&mut self, input: &[i64]) {
        self.inputs.extend_from_slice(input)
    }

    pub fn write_string(&mut self, input: &str) {
        self.inputs.extend(input.chars().map(|c| c as i64))
    }

    pub fn no_input(&self) -> bool {
        self.inputs.is_empty()
    }

    fn get(&self, pos: usize) -> i64 {
        if pos >= self.codes.len() { 0 } else { self.codes[pos] }
    }

    fn set(&mut self, pos: usize, val: i64) {
        if pos >= self.codes.len() {
            let extended_size = pos - self.codes.len() + 1;
            self.codes.extend(vec![0; extended_size]);
        }
        self.codes[pos] = val;
    }

    fn get_position(&self, mode: i64, param: usize) -> usize {
        let param_mode = (mode / 10_i64.pow(param as u32 + 1)) % 10;
        match param_mode {
            0 => self.get(self.pos + param) as usize,
            1 => self.pos + param,
            2 => (self.base as i64 + self.get(self.pos + param)) as usize,
            _ => panic!("wrong parameter mode {} at position {} for opcode {}", param_mode, self.pos, mode)
        }
    }

    fn get_param(&self, mode: i64, param: usize) -> i64 {
        self.get(self.get_position(mode, param))
    }

    pub fn interpreter(&mut self) -> bool {
        while self.pos < self.codes.len() {
            let mode = self.get(self.pos);
            let opcode = mode % 100;
            match opcode {
                1 => {
                    let res = self.get_position(mode,3);
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    self.set(res,val1 + val2);
                    self.pos += 4;
                },
                2 => {
                    let res = self.get_position(mode,3);
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    self.set(res, val1 * val2);
                    self.pos += 4;
                },
                3 => {
                    let res = self.get_position(mode,1);
                    if self.inputs.is_empty() {
                        return false;
                    } else {
                        let input = self.inputs.remove(0);
                        self.set(res, input);
                        self.pos += 2;
                    }
                },
                4 => {
                    self.outputs.push(self.get_param(mode, 1));
                    self.pos += 2;
                },
                5 => {
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    if val1 != 0 { self.pos = val2 as usize } else { self.pos += 3; };
                },
                6 => {
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    if val1 == 0 { self.pos = val2 as usize } else { self.pos += 3; };
                },
                7 => {
                    let res = self.get_position(mode, 3);
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    self.set(res, if val1 < val2 { 1 } else { 0 });
                    self.pos += 4;
                },
                8 => {
                    let res = self.get_position(mode,3);
                    let val1 = self.get_param(mode, 1);
                    let val2 = self.get_param(mode, 2);
                    self.set(res, if val1 == val2 { 1 } else { 0 });
                    self.pos += 4;
                },
                9 => {
                    self.base = (self.base as i64 + self.get_param(mode, 1)) as usize;
                    self.pos += 2;
                },
                99 => break,
                _ => panic!("wrong opcode {} at position {}", self.get(self.pos), self.pos)
            }
        }
        true
    }
}

pub fn extract_codes(contents: &str) -> Vec<i64> {
    contents
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}
