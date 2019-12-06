// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::VecDeque;
use std::io;

struct IntComp {
    values: Vec<isize>,
    pc: usize,
    output: Vec<isize>,
    input: VecDeque<isize>,
}

impl IntComp {
    fn get_vals(&mut self, mut opcode: usize, count: usize) -> Vec<isize> {
        let mut ret: Vec<isize> = self.values[self.pc..self.pc + count]
            .iter()
            .cloned()
            .collect();
        opcode /= 100;
        for v in ret.iter_mut() {
            if opcode & 1 == 0 {
                *v = self.values[*v as usize];
            }
            opcode /= 10;
        }
        self.pc += count;
        ret
    }
    fn add(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        let tgt = self.values[self.pc] as usize;
        self.pc += 1;
        self.values[tgt] = x[1] + x[0];
    }
    fn mult(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        let tgt = self.values[self.pc] as usize;
        self.pc += 1;
        self.values[tgt] = x[1] * x[0];
    }
    fn read(&mut self, cmd: usize) {
        let tgt = self.values[self.pc] as usize;
        self.pc += 1;
        self.values[tgt] = self.input.pop_front().unwrap();
    }
    fn write(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 1);
        self.output.push(x[0]);
    }
    fn jump_if_true(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        if x[0] != 0 {
            self.pc = x[1] as usize;
        }
    }
    fn jump_if_false(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        if x[0] == 0 {
            self.pc = x[1] as usize;
        }
    }
    fn less_than(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        let tgt = self.values[self.pc] as usize;
        self.pc += 1;
        self.values[tgt] = if x[0] < x[1] { 1 } else { 0 };
    }
    fn equals(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        let tgt = self.values[self.pc] as usize;
        self.pc += 1;
        self.values[tgt] = if x[0] == x[1] { 1 } else { 0 };
    }
    fn run(&mut self) -> Vec<isize> {
        loop {
            let cmd = self.values[self.pc] as usize;
            let cmd_byte = cmd % 100;
            self.pc += 1;
            // println!("OPCODE {} {} {}", cmd_byte, cmd, self.pc);
            match cmd_byte {
                1 => self.add(cmd),
                2 => self.mult(cmd),
                3 => self.read(cmd),
                4 => self.write(cmd),
                5 => self.jump_if_true(cmd),
                6 => self.jump_if_false(cmd),
                7 => self.less_than(cmd),
                8 => self.equals(cmd),
                _ => {
                    // println!("EXIT OPCODE {} {}", cmd_byte, cmd);
                    break;
                }
            };
        }
        self.output.clone()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let values: Vec<isize> = s
        .split(",")
        .map(|x| {
            let b: isize;
            scan!(x.bytes() => "{}", b);
            b
        })
        .collect();
    let mut ic = IntComp {
        values: values.clone(),
        pc: 0,
        input: vec![1].into_iter().collect(),
        output: Vec::new(),
    };
    ic.run();
    println!("{:?}", ic.output);
    let mut ic = IntComp {
        values: values.clone(),
        pc: 0,
        input: vec![5].into_iter().collect(),
        output: Vec::new(),
    };
    ic.run();
    println!("{:?}", ic.output);
}
