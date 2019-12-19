// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;

fn deploy_drone(ic: &IntComp, x: isize, y: isize) -> isize {
    let mut tilesic = ic.clone();
    tilesic.push_input(x);
    tilesic.push_input(y);
    tilesic.run_till_input();
    tilesic.pop_output()
}

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut grid = HashMap::new();
    for y in 0..50 {
        for x in 0..50 {
            grid.insert((x, y), deploy_drone(&ic, x, y));
        }
    }
    draw(&grid);
    println!("{}", grid.iter().filter(|(k, v)| **v == 1).count());
    let req_size = 100 - 1;
    let mut lastsp = (0, req_size + 1);
    loop {
        let y = lastsp.1 + 1;
        let mut x = lastsp.0;
        for x in lastsp.0.. {
            let points = [
                (x, y),
                (x + req_size, y),
                (x, y - req_size),
                (x + req_size, y - req_size),
            ];
            let mut values = points.iter().map(|p| deploy_drone(&ic, p.0, p.1));
            if values.next().unwrap() == 1 {
                if values.all(|v| v == 1) {
                    let tl = points[2];
                    println!("{} {} {}", tl.0, tl.1, tl.0 * 10000 + tl.1);
                    return;
                }
                lastsp = points[0];
                break;
            }
        }
    }
}

fn draw(gamestate: &HashMap<(isize, isize), isize>) {
    let minx = gamestate.keys().map(|x| x.0).min().unwrap();
    let miny = gamestate.keys().map(|x| x.1).min().unwrap();
    let maxx = gamestate.keys().map(|x| x.0).max().unwrap();
    let maxy = gamestate.keys().map(|x| x.1).max().unwrap();
    println!("{} {} {} {}", minx, maxx, miny, maxy);
    for y in miny..=maxy {
        for x in minx..=maxx {
            let v = *gamestate.get(&(x, y)).or(Some(&0)).unwrap();
            print!(
                "{}",
                match v {
                    0 => ".",
                    1 => "#",
                    _ => " ",
                }
            );
        }
        println!(" {}", y);
    }
}
