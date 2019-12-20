use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::str;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

const ID_DIRECTIONS: [((isize, isize), (isize, isize)); 4] = [
    ((0, -2), (0, -1)),
    ((0, 1), (0, 2)),
    ((-2, 0), (-1, 0)),
    ((1, 0), (2, 0)),
];

fn get_id(pos: (isize, isize), grid: &HashMap<(isize, isize), isize>) -> Option<String> {
    for d in ID_DIRECTIONS.iter() {
        let fp = (pos.0 + (d.0).0, pos.1 + (d.0).1);
        let sp = (pos.0 + (d.1).0, pos.1 + (d.1).1);
        let fpv = *grid.get(&fp).or(Some(&0)).unwrap();
        let spv = *grid.get(&sp).or(Some(&0)).unwrap();
        if fpv >= 'A' as isize && fpv <= 'Z' as isize && spv >= 'A' as isize && spv <= 'Z' as isize
        {
            let c = vec![fpv as u8, spv as u8];
            return Some(str::from_utf8(&c).unwrap().to_owned());
        }
    }
    None
}

fn find_by_id(id: String, grid: &HashMap<(isize, isize), isize>) -> Vec<(isize, isize)> {
    let mut results = Vec::new();
    let possible = grid.iter().filter(|(_k, v)| **v == '.' as isize);
    for z in possible {
        let val = get_id(*z.0, grid);
        match val {
            None => (),
            Some(p) => {
                if p == id {
                    results.push(*z.0);
                }
            }
        }
    }
    results
}

fn generate_others(
    grid: &HashMap<(isize, isize), isize>,
) -> HashMap<(isize, isize), (isize, isize, isize)> {
    let mut results = HashMap::new();
    let possible = grid.iter().filter(|(_k, v)| **v == '.' as isize);
    for z in possible {
        let val = get_id(*z.0, grid);
        match val {
            None => (),
            Some(_p) => {
                let o = find_other(((z.0).0, (z.0).1, 0), grid);
                if o != None {
                    results.insert(*z.0, o.unwrap());
                }
            }
        }
    }
    results
}

fn find_other(
    src: (isize, isize, isize),
    grid: &HashMap<(isize, isize), isize>,
) -> Option<(isize, isize, isize)> {
    let minx = grid.keys().map(|x| x.0).min().unwrap();
    let miny = grid.keys().map(|x| x.1).min().unwrap();
    let maxx = grid.keys().map(|x| x.0).max().unwrap();
    let maxy = grid.keys().map(|x| x.1).max().unwrap();
    let srcp = (src.0, src.1);
    let v = get_id(srcp, grid);
    if v == None {
        return None;
    }
    let others = find_by_id(v.unwrap(), grid);
    let mut outer = false;
    if srcp.0 == minx + 2 || srcp.0 == maxx - 2 {
        outer = true
    }
    if srcp.1 == miny + 2 || srcp.1 == maxy - 2 {
        outer = true
    }
    for x in others {
        if x != srcp {
            let nlevel = src.2 + if outer { 1 } else { -1 };
            return Some((x.0, x.1, nlevel));
        }
    }
    None
}

fn bfs(
    start: (isize, isize, isize),
    end: (isize, isize, isize),
    use_levels: bool,
    grid: &HashMap<(isize, isize), isize>,
    others: &HashMap<(isize, isize), (isize, isize, isize)>,
) -> isize {
    let mut cur = vec![start];
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut count = 0;
    let mut found = false;
    while !found {
        count += 1;
        let mut next = Vec::new();
        for x in cur.iter() {
            for d in DIRECTIONS.iter() {
                let np = (x.0 + d.0, x.1 + d.1, x.2);
                let spv = *grid.get(&(np.0, np.1)).or(Some(&0)).unwrap();
                if spv != '.' as isize {
                    continue;
                }
                if seen.insert(np) {
                    next.push(np);
                }
                if np == end {
                    found = true;
                }
            }
            let op = others.get(&(x.0, x.1));
            match op {
                Some(p) => {
                    let p = (p.0, p.1, if use_levels { x.2 + p.2 } else { 0 });
                    if (p.2) > 0 {
                        continue;
                    }
                    if seen.insert(p) {
                        next.push(p);
                    }
                    if p == end {
                        found = true;
                    }
                }
                None => (),
            }
        }
        cur = next;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let grid: HashMap<(isize, isize), isize> = stdin
        .lock()
        .lines()
        .enumerate()
        .map(|(line, input)| {
            input
                .unwrap()
                .chars()
                .enumerate()
                .map(|(x, v)| ((x as isize, line as isize), v as isize))
                .collect()
        })
        .collect::<Vec<Vec<((isize, isize), isize)>>>()
        .concat()
        .iter()
        .cloned()
        .collect();
    let start = find_by_id("AA".to_owned(), &grid)[0];
    let end = find_by_id("ZZ".to_owned(), &grid)[0];
    let others = generate_others(&grid);

    let count = bfs(
        (start.0, start.1, 0),
        (end.0, end.1, 0),
        false,
        &grid,
        &others,
    );
    println!("{:?}", count);

    let count = bfs(
        (start.0, start.1, 0),
        (end.0, end.1, 0),
        true,
        &grid,
        &others,
    );
    println!("{:?}", count);
}
