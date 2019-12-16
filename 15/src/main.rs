// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;
use std::collections::HashSet;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
fn bfs(
    grid: &HashMap<(isize, isize), isize>,
    loc: (isize, isize),
    target: Option<(isize, isize)>,
) -> (isize, isize) {
    let mut seen = HashSet::new();
    let mut count = 0;
    let mut cur = vec![(loc, -1)];
    while cur.is_empty() == false {
        count += 1;
        let mut next = Vec::new();
        for x in cur {
            for (i, p) in DIRECTIONS.iter().enumerate() {
                let np = ((x.0).0 + p.0, (x.0).1 + p.1);
                let dir = if x.1 >= 0 { x.1 } else { i as isize };
                if seen.contains(&np) {
                    continue;
                }
                let cell = *grid.get(&np).or(Some(&-1)).unwrap();
                match target {
                    Some(p) => {
                        if p == np {
                            return (dir, count);
                        }
                    }
                    None => {
                        if cell == -1 {
                            return (dir, 1);
                        }
                    }
                }
                if cell == 0 {
                    continue;
                }
                seen.insert(np);
                next.push((np, dir));
            }
        }
        cur = next;
    }
    (-1, count - 1)
}

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut tilesic = ic.clone();
    let mut last_move = 0;
    let mut grid = HashMap::new();
    let mut loc = (0, 0);
    let mut tgt = (0, 0);
    grid.insert(loc, 3);
    while !tilesic.finished() {
        tilesic.push_input(last_move + 1);
        tilesic.run_till_input();
        if tilesic.output_len() >= 1 {
            let a = tilesic.pop_output();
            let next = (
                loc.0 + DIRECTIONS[last_move as usize].0,
                loc.1 + DIRECTIONS[last_move as usize].1,
            );
            grid.insert(next, a);
            if a != 0 {
                loc = next;
            }
            if a == 2 {
                tgt = next;
            }
            let (dir, _count) = bfs(&grid, loc, None);
            last_move = dir;
            if a < 0 {
                break;
            }
        }
    }
    draw(&grid);
    println!("{:?}", bfs(&grid, (0, 0), Some(tgt)).1);
    let p2 = bfs(&grid, tgt, None);
    println!("{:?}", p2.1);
}

fn draw(gamestate: &HashMap<(isize, isize), isize>) {
    let minx = gamestate.keys().map(|x| x.0).min().unwrap();
    let miny = gamestate.keys().map(|x| x.1).min().unwrap();
    let maxx = gamestate.keys().map(|x| x.0).max().unwrap();
    let maxy = gamestate.keys().map(|x| x.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let v = *gamestate.get(&(x, y)).or(Some(&-1)).unwrap();
            print!(
                "{}",
                match v {
                    0 => "#",
                    1 => ".",
                    2 => "?",
                    3 => "*",
                    _ => " ",
                }
            );
        }
        println!();
    }
}
