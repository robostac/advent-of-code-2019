#[macro_use]
extern crate text_io;
use std::io;

fn count(cur_prefix: usize, last_value: usize, rem: usize, valid: &dyn Fn(usize) -> bool) -> usize {
    if rem == 0 {
        if valid(cur_prefix) {
            1
        } else {
            0
        }
    } else {
        (last_value..=9)
            .map(|x| count((cur_prefix * 10) + x, x, rem - 1, valid))
            .sum()
    }
}

fn get_digits(mut x: usize) -> [usize; 10] {
    let mut counts = [0; 10];
    while x > 0 {
        counts[x % 10] += 1;
        x /= 10;
    }
    counts
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let values: Vec<usize> = s
        .split("-")
        .map(|x| {
            let b: usize;
            scan!(x.bytes() => "{}", b);
            b
        })
        .collect();
    let part1 = |x| {
        if x < values[0] || x > values[1] {
            false
        } else {
            get_digits(x).iter().any(|&x| x >= 2)
        }
    };
    println!("{:?}", count(0, 0, 6, &part1));

    let part2 = |x| {
        if x < values[0] || x > values[1] {
            false
        } else {
            get_digits(x).iter().any(|&x| x == 2)
        }
    };
    println!("{:?}", count(0, 0, 6, &part2));
}
