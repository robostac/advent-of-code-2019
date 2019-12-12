// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn update_vel(&mut self, other: &Moon) {
        for i in 0..3 {
            let c = self.pos[i] - other.pos[i];
            self.vel[i] -= c.signum();
        }
    }
    fn step(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn energy(&self) -> i64 {
        let e: i64 = self.pos.iter().map(|x| x.abs()).sum();
        let ke: i64 = self.vel.iter().map(|x| x.abs()).sum();
        ke * e
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn main() {
    let stdin = io::stdin();
    let mut values: Vec<Moon> = stdin
        .lock()
        .lines()
        .map(|input| {
            let x: i64;
            let y: i64;
            let z: i64;
            let s = input.unwrap();
            scan!(s.bytes() => "<x={}, y={}, z={}>", x,y,z);
            Moon {
                pos: [x, y, z],
                vel: [0, 0, 0],
            }
        })
        .collect();
    let mut hm: Vec<HashMap<(i64, i64), i64>> = (0..12).map(|_x| HashMap::new()).collect();
    let mut cl = [0i64, 0, 0];
    for iter in 1.. {
        values = values
            .iter()
            .enumerate()
            .map(|(p, x)| {
                let mut x = x.clone();
                for (j, y) in values.iter().enumerate() {
                    if j != p {
                        x.update_vel(y);
                    }
                }
                x.step();
                x
            })
            .collect();
        let mut all = true;
        for i in 0..3 {
            if cl[i] == 0 {
                all = false;
                let seen = values
                    .iter()
                    .enumerate()
                    .map(|(p, x)| *hm[i * 4 + p].entry((x.pos[i], x.vel[i])).or_insert(iter))
                    .collect::<HashSet<i64>>();
                let last = *seen.iter().next().unwrap();
                if seen.len() == 1 && last != iter {
                    println!("{} {}", i, iter - last);
                    cl[i] = iter - last;
                }
            }
        }
        if all {
            let p1 = cl[0] / gcd(cl[1], cl[0]) * cl[1];
            let p2 = p1 / gcd(p1, cl[2]) * cl[2];
            println!("{:?} {}", cl, p2);
            break;
        }
        if iter == 1000 {
            let te: i64 = values.iter().map(|x| x.energy()).sum();
            println!("{}", te);
        }
    }
}
