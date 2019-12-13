#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

pub struct Computer {
    data: Vec<i64>,
    offset: usize,
    halted: bool,
}

impl Computer {
    pub fn new(data: Vec<i64>) -> Self {
        Self {
            data,
            offset: 0,
            halted: false,
        }
    }

    pub fn run(&mut self, mut input: Vec<i64>) -> Vec<i64> {
        let mut output = Vec::new();

        let mut digits = vec![0; 5]; //reusable buffer for opcodes and modes
        loop {
            for d in &mut digits {
                //clear buffer
                *d = 0;
            }
            get_digits(self.data[self.offset], 0, &mut digits); // populate buffer with opcode and then mode digits
            let opcode = &digits[..2];
            let modes = &digits[2..];
            self.offset = match opcode {
                // match on opcode
                [1, 0] => {
                    // add
                    self.add(modes);
                    self.offset + 4
                }
                [2, 0] => {
                    // multiply
                    self.mul(modes);
                    self.offset + 4
                }
                [3, 0] => {
                    // input
                    if input.is_empty() {
                        break;
                    }
                    let offset = self.data[self.offset + 1] as usize;
                    self.data[offset] = input.remove(0);
                    self.offset + 2
                }
                [4, 0] => {
                    // output
                    output.push(self.output(modes));
                    self.offset + 2
                }
                [5, 0] => {
                    // jump if true
                    self.jit(modes)
                }
                [6, 0] => {
                    // jump if false
                    self.jif(modes)
                }
                [7, 0] => {
                    // less than
                    self.lt(modes);
                    self.offset + 4
                }
                [8, 0] => {
                    // equals
                    self.eq(modes);
                    self.offset + 4
                }
                [9, 9] => {
                    self.halted = true;
                    break;
                }
                _ => panic!(
                    "Unexpected opcode {} at {}",
                    self.data[self.offset], self.offset
                ),
            };
        }

        output
    }

    pub fn data(&self) -> &[i64] {
        &self.data
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    // Gets param values from `data` starting at `offset`
    // `offset` should point to first param
    // Will get a param for each paramater modes in `modes`
    fn get_params(&self, offset: usize, modes: &[i64]) -> Vec<i64> {
        let mut params = Vec::new();
        for (i, &mode) in modes.iter().enumerate() {
            let mut param = self.data[offset + i];
            if mode == 0 {
                param = self.data[param as usize];
            }
            params.push(param);
        }
        params
    }

    fn add(&mut self, modes: &[i64]) {
        let params = self.get_params(self.offset + 1, &modes[..2]); // get 2 params from data, starting at offset + 1
        let output = self.data[self.offset + 3] as usize;
        self.data[output] = params[0] + params[1];
    }

    fn mul(&mut self, modes: &[i64]) {
        let params = self.get_params(self.offset + 1, &modes[..2]);
        let output = self.data[self.offset + 3] as usize;
        self.data[output] = params[0] * params[1];
    }

    fn output(&self, modes: &[i64]) -> i64 {
        let params = self.get_params(self.offset + 1, &modes[..1]); // get 1 param from data, starting at offset + 1
        params[0]
    }

    // if first param is non-zero, return 2nd param as new offset, otherwise do nothing (return current offset)
    fn jit(&self, modes: &[i64]) -> usize {
        let params = self.get_params(self.offset + 1, &modes[..2]);

        if params[0] == 0 {
            self.offset + 3
        } else {
            params[1] as usize
        }
    }

    // if first param is zero, return 2nd param as new offset, otherwise advance as usual
    fn jif(&self, modes: &[i64]) -> usize {
        let params = self.get_params(self.offset + 1, &modes[..2]);

        if params[0] == 0 {
            params[1] as usize
        } else {
            self.offset + 3
        }
    }

    fn lt(&mut self, modes: &[i64]) {
        let params = self.get_params(self.offset + 1, &modes[..2]);
        let output = self.data[self.offset + 3] as usize;
        self.data[output] = if params[0] < params[1] { 1 } else { 0 };
    }

    fn eq(&mut self, modes: &[i64]) {
        let params = self.get_params(self.offset + 1, &modes[..2]);
        let output = self.data[self.offset + 3] as usize;
        self.data[output] = if params[0] == params[1] { 1 } else { 0 };
    }
}

fn get_digits(d: i64, i: usize, v: &mut [i64]) {
    if d >= 10 {
        get_digits(d / 10, i + 1, v);
    }
    v[i] = d % 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_pos() {
        let data = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut eq = Computer::new(data.clone());
        assert_eq!(vec![1], eq.run(vec![8]));

        let mut neq = Computer::new(data);
        assert_eq!(vec![0], neq.run(vec![9]));
    }

    #[test]
    fn lt_pos() {
        let data = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut lt = Computer::new(data.clone());
        assert_eq!(vec![1], lt.run(vec![7]));

        let mut nlt = Computer::new(data);
        assert_eq!(vec![0], nlt.run(vec![8]));
    }

    #[test]
    fn eq_imm() {
        let data = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut eq = Computer::new(data.clone());
        assert_eq!(vec![1], eq.run(vec![8]));

        let mut neq = Computer::new(data);
        assert_eq!(vec![0], neq.run(vec![9]));
    }

    #[test]
    fn lt_imm() {
        let data = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut lt = Computer::new(data.clone());
        assert_eq!(vec![1], lt.run(vec![7]));

        let mut nlt = Computer::new(data);
        assert_eq!(vec![0], nlt.run(vec![8]));
    }

    #[test]
    fn amp1() {
        let data = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let mut output = vec![0];
        let phases = &[4, 3, 2, 1, 0];

        for phase in phases {
            let mut amp = Computer::new(data.clone());
            output = amp.run(vec![*phase, output[0]]);
        }

        assert_eq!(vec![43210], output);
    }

    #[test]
    fn amp2() {
        let data = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        let mut output = vec![0];
        let phases = &[0, 1, 2, 3, 4];

        for phase in phases {
            let mut amp = Computer::new(data.clone());
            output = amp.run(vec![*phase, output[0]]);
        }

        assert_eq!(vec![54321], output);
    }

    #[test]
    fn amp3() {
        let data = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        let mut output = vec![0];
        let phases = &[1, 0, 4, 3, 2];

        for phase in phases {
            let mut amp = Computer::new(data.clone());
            output = amp.run(vec![*phase, output[0]]);
        }

        assert_eq!(vec![65210], output);
    }
}
