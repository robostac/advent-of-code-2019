use modinverse::modinverse;
use std::io;
use std::io::prelude::*;

fn shuffle(ncards: i64, mut pos: i64, inst: &Vec<Vec<String>>) -> i64 {
    for x in inst.iter() {
        if x[0] == "cut" {
            let v = x[1].parse::<i64>().unwrap();
            pos = (pos - v) % ncards;
        } else if x[0] == "deal" && x[1] == "with" {
            let v = x[3].parse::<i64>().unwrap();
            pos = (pos * v) % ncards
        } else if x[0] == "deal" && x[1] == "into" {
            pos = ncards - 1 - pos;
        }
    }
    pos
}

fn rev_shuffle(ncards: i64, mut pos: i64, inst: &Vec<Vec<String>>) -> i64 {
    for x in inst.iter().rev() {
        if x[0] == "cut" {
            let v = x[1].parse::<i64>().unwrap();
            pos = (pos + v) % ncards;
        } else if x[0] == "deal" && x[1] == "with" {
            let v = x[3].parse::<i64>().unwrap();
            let mut z = pos;
            while z % v != 0 {
                z += ncards;
            }
            pos = (z / v) % ncards;
        } else if x[0] == "deal" && x[1] == "into" {
            pos = ncards - 1 - pos;
        }
    }
    pos
}

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn apply(x: &Vec<String>, a: i128, b: i128, modval: i128) -> (i128, i128) {
    if x[0] == "cut" {
        let v = x[1].parse::<i128>().unwrap();
        (a, (b - v) % modval)
    } else if x[1] == "into" {
        (-a % modval, (modval - 1 - b) % modval)
    } else {
        let a128 = a as i128;
        let b128 = b as i128;
        let m128 = modval as i128;
        let v = x[3].parse::<i128>().unwrap();
        (((a128 * v) % m128) as i128, ((b128 * v) % m128) as i128)
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<String>> = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap().split(" ").map(|x| x.to_owned()).collect())
        .collect();
    println!("{:?}", shuffle(10007, 2019, &values));
    let m = 119315717514047i128;
    let cards = 2020i128;
    let n = 101741582076661i128;
    let mut a = 1i128;
    let mut b = 0i128;
    for x in values.iter() {
        let c = apply(&x, a, b, m);
        a = c.0;
        b = c.1;
    }
    let modinv = modinverse(1 - a, m).unwrap();
    let r = (b * modinv) % m;
    let pos = (((cards - r) * mod_pow(a, n * (m - 2), m) + r) + m) % m;
    println!(" {:?}", pos);
}
