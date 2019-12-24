// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;
use std::collections::HashSet;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn run(inst: &[&str], ic: &IntComp) -> Option<isize> {
    let mut tilesic = ic.clone();
    let mut grid = HashMap::new();
    for p in inst.iter() {
        for x in p.bytes() {
            tilesic.push_input(x as isize);
        }
        tilesic.push_input('\n' as isize);
    }
    tilesic.run();
    let mut x = 0;
    let mut y = 0;
    for v in tilesic.output_ref() {
        if *v > 1000 {
            draw(&grid);
            return Some(*v);
        }
        if *v == 10 {
            y += 1;
            x = 0;
        } else {
            grid.insert((x, y), *v);
            x += 1;
        }
    }
    draw(&grid);
    None
}
fn main() {
    let ic = IntComp::build_from_stdin();
    let inst = [
        "NOT D J", "NOT J J", "OR A T", "AND A T", "AND B T", "AND C T", "NOT T T", "AND T J",
        "WALK",
    ];
    println!("{:?}", run(&inst, &ic));
    let inst = [
        "NOT E T", "NOT T T", "OR H T", "AND D T", "NOT T J", "NOT J J", "OR A T", "AND A T",
        "AND B T", "AND C T", "NOT T T", "AND T J", "RUN",
    ];
    println!("{:?}", run(&inst, &ic));
}

fn draw(gamestate: &HashMap<(isize, isize), isize>) {
    let minx = gamestate.keys().map(|x| x.0).min().unwrap();
    let miny = gamestate.keys().map(|x| x.1).min().unwrap();
    let maxx = gamestate.keys().map(|x| x.0).max().unwrap();
    let maxy = gamestate.keys().map(|x| x.1).max().unwrap();
    println!("{} {} {} {}", minx, maxx, miny, maxy);
    for y in miny..=maxy {
        for x in minx..=maxx {
            let v = *gamestate.get(&(x, y)).or(Some(&32)).unwrap();

            print!("{}", v as u8 as char);
        }
        println!();
    }
}
