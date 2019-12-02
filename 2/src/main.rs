// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::io;

fn run_comp(vec: &Vec<usize>, a: usize, b: usize) -> usize {
    let mut values = vec.clone();
    let mut pc: usize = 0;
    values[1] = a;
    values[2] = b;
    while values[pc] != 99 {
        let (a, b, tgt) = (
            values[values[pc + 1]],
            values[values[pc + 2]],
            values[pc + 3],
        );
        // println!("{} {} {} {}", values[pc], tgt, a, b);
        match values[pc] {
            1 => values[tgt] = a + b,
            2 => values[tgt] = a * b,
            _ => println!("Invalid Opcode {}", values[pc]),
        }
        pc += 4;
    }
    values[0]
}
fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let mut values: Vec<usize> = s
        .split(",")
        .map(|x| {
            let b: usize;
            scan!(x.bytes() => "{}", b);
            b
        })
        .collect();
    println!("{:?}", run_comp(&values, 12, 2));
    for a in 0..99 {
        for b in 0..99 {
            if (run_comp(&values, a, b)) == 19690720 {
                println!("{} {} {}", a, b, a * 100 + b)
            }
        }
    }
}
