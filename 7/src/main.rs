// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::VecDeque;
use std::io;
#[derive(Clone, Debug)]
struct IntComp {
    values: Vec<isize>,
    pc: usize,
    output: VecDeque<isize>,
    input: VecDeque<isize>,
    finished: bool,
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
        self.output.push_back(x[0]);
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
    fn run(&mut self) {
        loop {
            let cmd = self.values[self.pc] as usize;
            let cmd_byte = cmd % 100;
            self.pc += 1;
            // println!("OPCODE {} {} {}", cmd_byte, cmd, self.pc);
            match cmd_byte {
                1 => self.add(cmd),
                2 => self.mult(cmd),
                3 => self.read(cmd),
                4 => {
                    self.write(cmd);
                    break;
                }
                5 => self.jump_if_true(cmd),
                6 => self.jump_if_false(cmd),
                7 => self.less_than(cmd),
                8 => self.equals(cmd),
                _ => {
                    self.finished = true;
                    // println!("EXIT OPCODE {} {}", cmd_byte, cmd);
                    break;
                }
            };
        }
    }
}

fn permute(comp: &IntComp, used: usize, mut input: [isize; 5], cur: usize) -> isize {
    if cur == 5 {
        let mut val = 0;
        for x in input.iter() {
            let mut c = comp.clone();
            c.input.push_back(*x);
            c.input.push_back(val);
            c.run();
            val = c.output[0];
        }
        return val;
    }
    (0..5)
        .filter_map(|x| {
            if used & (1 << x) == 0 {
                input[cur] = x;
                Some(permute(comp, used | 1 << x, input, cur + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn permute2(comp: &IntComp, used: usize, mut input: [isize; 5], cur: usize) -> isize {
    if cur == 5 {
        let mut comps: Vec<IntComp> = vec![comp.clone(); 5];
        for i in 0..5 {
            comps[i].input.push_back(input[i]);
        }
        let mut val = 0;
        while comps[4].finished == false {
            for x in 0..5 {
                comps[x].input.push_back(val);
                comps[x].run();
                if comps[x].finished == false {
                    val = comps[x].output.pop_front().unwrap();
                }
            }
        }
        return val;
    }
    (5..10)
        .filter_map(|x| {
            if used & (1 << x) == 0 {
                input[cur] = x;
                Some(permute2(comp, used | 1 << x, input, cur + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap()
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
    let ic = IntComp {
        values: values.clone(),
        pc: 0,
        input: VecDeque::new(),
        output: VecDeque::new(),
        finished: false,
    };
    println!("{}", permute(&ic, 0, [0, 0, 0, 0, 0], 0));

    println!("{}", permute2(&ic, 0, [0, 0, 0, 0, 0], 0));
}
