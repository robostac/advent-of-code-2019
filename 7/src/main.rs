// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
#[derive(Clone, Debug)]
struct IntComp {
    values: HashMap<usize, isize>,
    pc: usize,
    rel: isize,
    output: VecDeque<isize>,
    input: VecDeque<isize>,
    finished: bool,
}

impl IntComp {
    fn get_vals(&mut self, mut opcode: usize, count: usize) -> Vec<(isize, Option<usize>)> {
        let mut ret: Vec<(isize, Option<usize>)> = (0..count)
            .map(|x| (*self.values.entry(self.pc + x).or_insert(0), None))
            .collect();
        opcode /= 100;
        for v in ret.iter_mut() {
            match opcode % 10 {
                0 => {
                    *v = (
                        *self.values.entry(v.0 as usize).or_insert(0),
                        Some(v.0 as usize),
                    )
                }
                1 => (),
                2 => {
                    *v = (
                        *self.values.entry((self.rel + v.0) as usize).or_insert(0),
                        Some((self.rel + v.0) as usize),
                    )
                }
                _ => panic!("Unknown Mode {}", opcode % 10),
            }
            opcode /= 10;
        }
        self.pc += count;
        ret
    }
    fn add(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 3);
        *self.values.entry(x[2].1.unwrap()).or_insert(0) = x[1].0 + x[0].0;
    }
    fn mult(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 3);
        *self.values.entry(x[2].1.unwrap()).or_insert(0) = x[1].0 * x[0].0;
    }
    fn read(&mut self, cmd: usize) {
        if self.input.is_empty() {
            self.pc -= 1;
        } else {
            let x = self.get_vals(cmd, 1);
            *self.values.entry(x[0].1.unwrap()).or_insert(0) = self.input.pop_front().unwrap();
        }
    }
    fn write(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 1);
        self.output.push_back(x[0].0);
    }
    fn jump_if_true(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        if x[0].0 != 0 {
            self.pc = x[1].0 as usize;
        }
    }
    fn jump_if_false(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 2);
        if x[0].0 == 0 {
            self.pc = x[1].0 as usize;
        }
    }
    fn less_than(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 3);
        *self.values.entry(x[2].1.unwrap()).or_insert(0) = if x[0].0 < x[1].0 { 1 } else { 0 };
    }
    fn equals(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 3);
        *self.values.entry(x[2].1.unwrap()).or_insert(0) = if x[0].0 == x[1].0 { 1 } else { 0 };
    }
    fn rel_update(&mut self, cmd: usize) {
        let x = self.get_vals(cmd, 1);
        self.rel += x[0].0
    }

    fn step(&mut self) {
        if self.finished {
            return;
        }
        let cmd = self.values[&self.pc] as usize;
        let cmd_byte = cmd % 100;
        self.pc += 1;
        //println!("OPCODE {} {} {} {}", cmd_byte, cmd, self.pc, self.rel);
        match cmd_byte {
            1 => self.add(cmd),
            2 => self.mult(cmd),
            3 => self.read(cmd),
            4 => self.write(cmd),
            5 => self.jump_if_true(cmd),
            6 => self.jump_if_false(cmd),
            7 => self.less_than(cmd),
            8 => self.equals(cmd),
            9 => self.rel_update(cmd),
            99 => self.finished = true,
            _ => panic!("Unknown Opcode {}", cmd_byte),
        };
    }

    fn run(&mut self) {
        while self.finished == false {
            self.step();
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
        comps[0].input.push_back(0);
        while comps[4].finished == false {
            for x in 0..5 {
                comps[x].step();
                let copy = &comps[x].output.clone();
                comps[(x + 1) % 5].input.extend(copy);
                comps[x].output.clear();
            }
        }
        return comps[0].input.pop_front().unwrap();
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
        values: values.iter().cloned().enumerate().collect(),
        pc: 0,
        rel: 0,
        input: VecDeque::new(),
        output: VecDeque::new(),
        finished: false,
    };
    println!("{}", permute(&ic, 0, [0, 0, 0, 0, 0], 0));

    println!("{}", permute2(&ic, 0, [0, 0, 0, 0, 0], 0));
}
