use intcomp::IntComp;
use std::collections::HashMap;

fn draw_pong(gamestate: &HashMap<(isize, isize), isize>) {
    let minx = gamestate.keys().map(|x| x.0).min().unwrap();
    let miny = gamestate.keys().map(|x| x.1).min().unwrap();
    let maxx = gamestate.keys().map(|x| x.0).max().unwrap();
    let maxy = gamestate.keys().map(|x| x.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let v = *gamestate.get(&(x, y)).or(Some(&0)).unwrap();
            print!(
                "{}",
                match v {
                    1 => "#",
                    2 => "@",
                    3 => "_",
                    4 => "*",
                    _ => " ",
                }
            );
        }
        println!();
    }
}

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut tilesic = ic.clone();
    tilesic.run();
    let tiles = tilesic
        .output_ref()
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&&x| x == 2)
        .count();
    println!("{}", tiles);

    let mut gamestate = tilesic
        .output_ref()
        .iter()
        .cloned()
        .collect::<Vec<isize>>()
        .chunks(3)
        .map(|x| ((x[0], x[1]), x[2]))
        .collect::<HashMap<(isize, isize), isize>>();
    let mut tilesic = ic.clone();
    *tilesic.values.entry(0).or_insert(0) = 2;
    let mut ball = (0, 0);
    let mut pad = (0, 0);
    let mut score = 0;

    while !tilesic.finished() {
        tilesic.run_till_input();
        while tilesic.output_len() >= 3 {
            let a = tilesic.pop_output();
            let b = tilesic.pop_output();
            let c = tilesic.pop_output();
            *gamestate.entry((a, b)).or_insert(0) = c;
            if c == 3 {
                pad = (a, b);
            }
            if c == 4 {
                ball = (a, b)
            }
            if a < 0 {
                score = c;
                // draw_pong(&gamestate);
            }
        }
        tilesic.push_input((ball.0 - pad.0).signum());
    }
    draw_pong(&gamestate);
    println!("{}", score);
}
