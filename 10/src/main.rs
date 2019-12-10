use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(PartialEq, PartialOrd)]
struct NonNan(f64);

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

const PI2: f64 = std::f64::consts::PI * 2.0;
fn angle(diff: &(isize, isize)) -> f64 {
    ((diff.1 as f64).atan2(diff.0 as f64))
}

fn main() {
    let stdin = io::stdin();
    let mut grid = HashSet::new();
    for (y, input) in stdin.lock().lines().enumerate() {
        for (x, v) in input.unwrap().as_bytes().iter().enumerate() {
            if *v == '#' as u8 {
                grid.insert((x as isize, y as isize));
            }
        }
    }
    let best = grid
        .iter()
        .map(|src| {
            let mut visible: HashMap<u64, usize> = HashMap::new();
            for dest in grid.iter().filter(|x| *x != src) {
                let diff = (src.0 - dest.0, src.1 - dest.1);
                let key = angle(&diff); //theres a chance this doesn't work due to fp errors
                *visible.entry(key.to_bits()).or_default() += 1;
            }
            (visible, *src)
        })
        .max_by_key(|x| x.0.len())
        .unwrap();
    println!("{}", best.0.len());
    grid.remove(&best.1);
    const EPS: f64 = 0.0000001f64;
    let mut ang = (std::f64::consts::FRAC_PI_2) - EPS;
    let src = best.1;
    let mut count = 0;
    while grid.is_empty() == false {
        count += 1;
        let next = grid
            .iter()
            .min_by_key(|x| {
                let diff = (src.0 - x.0, src.1 - x.1);
                (
                    NonNan((angle(&diff) - ang + PI2) % PI2),
                    diff.0 * diff.0 + diff.1 * diff.1,
                )
            })
            .unwrap()
            .clone();
        let diff = (src.0 - next.0, src.1 - next.1);
        ang = angle(&diff) + EPS;
        if count == 200 {
            println!("{:?} {}", next, next.0 * 100 + next.1);
        }
        grid.remove(&next);
    }
}
