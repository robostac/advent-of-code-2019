// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let x: i64;
            let s = input.unwrap();
            scan!(s.bytes() => "{}", x);
            x
        })
        .collect();
    println!("{:?}", values);
}
