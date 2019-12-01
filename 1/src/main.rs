// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::io;
use std::io::prelude::*;

fn fuel(x: i64) -> i64 {
    x / 3 - 2
}

fn cost(x: i64) -> i64 {
    if x > 0 {
        x + cost(fuel(x))
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let x: i64;
            let s = input.unwrap();
            scan!(s.bytes() => "{}", x);
            fuel(x)
        })
        .collect();
    let total: i64 = values.iter().sum();
    println!("{:?}", total);

    let total2: i64 = values.iter().map(|&x| cost(x)).sum();
    println!("{:?}", total2);
}
