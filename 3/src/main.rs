use std::io;
use std::io::prelude::*;

fn dir_distance(instr: &str) -> (isize, isize) {
    let dir = instr.chars().nth(0).unwrap();
    let dist = instr[1..].parse::<isize>().unwrap();

    let d = match dir {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => (0, 0),
    };
    (d.0 * dist, d.1 * dist)
}

fn is_inside(a: isize, b: isize, c: isize) -> bool {
    c >= std::cmp::min(a, b) && c <= std::cmp::max(a, b)
}

fn cross(
    p1: (isize, isize),
    p2: (isize, isize),
    p3: (isize, isize),
    p4: (isize, isize),
) -> Option<(isize, isize)> {
    if is_inside(p1.1, p2.1, p3.1) && is_inside(p3.0, p4.0, p1.0) {
        Some((p1.0, p3.1))
    } else if is_inside(p1.0, p2.0, p3.0) && is_inside(p3.1, p4.1, p1.1) {
        Some((p3.0, p1.1))
    } else {
        None
    }
}

fn dist(p1: (isize, isize), p2: (isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<Vec<(isize, isize)>> = stdin
        .lock()
        .lines()
        .map(|input| {
            let mut next_point: (isize, isize) = (0, 0);
            input
                .unwrap()
                .split(",")
                .map(|x| {
                    let dir = dir_distance(x);
                    next_point = (next_point.0 + dir.0, next_point.1 + dir.1);
                    next_point
                })
                .collect()
        })
        .collect();
    let mut dd1 = 0;
    let mut sp1 = (0, 0);
    let y = points[0]
        .windows(2)
        .map(|p| {
            dd1 += dist(sp1, p[0]);
            sp1 = p[0];
            let mut sp2 = (0, 0);
            let mut dd2 = 0;
            points[1]
                .windows(2)
                .filter_map(|x| {
                    dd2 += dist(sp2, x[0]);
                    sp2 = x[0];
                    match cross(p[0], p[1], x[0], x[1]) {
                        Some(np) => Some((np, dd1 + dd2 + dist(np, p[0]) + dist(np, x[0]))),
                        None => None,
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<((isize, isize), isize)>>>()
        .concat();
    println!("{}", y.iter().map(|&x| dist(x.0, (0, 0))).min().unwrap());
    println!("{}", y.iter().map(|&x| x.1).min().unwrap());
}
