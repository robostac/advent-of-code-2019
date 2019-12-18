use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    cost: usize,
    position: [(isize, isize); 4],
    keys: usize,
    pcount: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.keys.cmp(&other.keys))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(loc: (isize, isize), keys: usize, cost: usize) -> State {
        State {
            cost: cost,
            keys: keys,
            position: [loc, (0, 0), (0, 0), (0, 0)],
            pcount: 1,
        }
    }
    fn new4(loc: [(isize, isize); 4], keys: usize, cost: usize) -> State {
        State {
            cost: cost,
            keys: keys,
            position: loc,
            pcount: 4,
        }
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
fn find_keys(
    grid: &HashMap<(isize, isize), char>,
    loc: (isize, isize),
    cur_keys: usize,
) -> Vec<((isize, isize), usize, usize)> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    let mut count = 0;
    let mut cur = vec![(loc)];
    while cur.is_empty() == false {
        count += 1;
        let mut next = Vec::new();
        for x in cur {
            for p in DIRECTIONS.iter() {
                let np = (x.0 + p.0, x.1 + p.1);
                let cell = *grid.get(&np).or(Some(&'#')).unwrap();
                let mut door = false;
                let mut key = false;
                let mut key_val = 0;
                match cell {
                    '.' => (),
                    '#' => continue,
                    '@' => (),
                    p => {
                        door = ((p as u8) & 0x20) == 0;
                        key = !door;
                        key_val = 1 << ((p as usize) & !0x20) + 1 - ('A' as usize);
                    }
                }
                if door {
                    if (cur_keys & key_val) == 0 {
                        continue;
                    }
                }
                if seen.insert(np) {
                    if key && cur_keys & key_val == 0 {
                        results.push((np, cur_keys | key_val, count));
                        continue;
                    }
                    next.push(np);
                }
            }
        }
        cur = next;
    }
    results
}

fn main() {
    let stdin = io::stdin();
    let grid: HashMap<(isize, isize), char> = stdin
        .lock()
        .lines()
        .enumerate()
        .map(|(line, input)| {
            input
                .unwrap()
                .chars()
                .enumerate()
                .map(|(x, v)| ((x as isize, line as isize), v))
                .collect()
        })
        .collect::<Vec<Vec<((isize, isize), char)>>>()
        .concat()
        .iter()
        .cloned()
        .collect();
    let start = grid.iter().find(|(_k, v)| **v == '@').unwrap().0;
    println!("{:?}", search(&grid, State::new(*start, 0, 0)));

    let mut grid2 = grid.clone();

    for x in -1..=1 {
        for y in -1..=1 {
            grid2.insert((start.0 + x, start.1 + y), '#');
        }
    }
    let starts = [
        (start.0 - 1, start.1 - 1),
        (start.0 + 1, start.1 - 1),
        (start.0 - 1, start.1 + 1),
        (start.0 + 1, start.1 + 1),
    ];
    for x in starts.iter() {
        grid2.insert(*x, '@');
    }
    println!("{:?}", search(&grid2, State::new4(starts, 0, 0)));
}

fn search(grid: &HashMap<(isize, isize), char>, initial: State) -> (usize) {
    let mut heap = BinaryHeap::new();
    heap.push(initial);
    let nkeys: u32 = grid
        .iter()
        .filter(|(_k, v)| **v >= 'a' && **v <= 'z')
        .count() as u32;
    let mut seen = HashSet::new();
    let mut best = std::usize::MAX;
    while heap.is_empty() == false {
        let cur = heap.pop().unwrap();
        if cur.cost >= best {
            continue;
        }
        if seen.insert((cur.position, cur.keys)) == false {
            continue;
        }
        for i in 0..cur.pcount {
            let next_states = find_keys(grid, cur.position[i], cur.keys);
            for x in next_states.iter() {
                let mut np = cur.position.clone();
                np[i] = x.0;
                heap.push(State::new4(np, x.1, x.2 + cur.cost));
                if x.1.count_ones() == nkeys {
                    if x.2 + cur.cost < best {
                        best = x.2 + cur.cost;
                    }
                }
            }
        }
    }
    best
}
