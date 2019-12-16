// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;
fn run_robot(ic: &IntComp, initial: isize) -> HashMap<(isize, isize), isize> {
    let mut tempic = ic.clone();
    let mut panels = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = (0, -1);
    panels.insert(pos, initial);
    loop {
        let cur = panels.entry(pos).or_insert(0);
        tempic.push_input(*cur);
        tempic.run_till_input();
        if tempic.finished() {
            return panels;
        }
        *cur = tempic.pop_output();
        match tempic.pop_output() {
            0 => dir = (dir.1, -dir.0),
            1 => dir = (-dir.1, dir.0),
            _ => panic!("Unknown direction change"),
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn main() {
    let ic = IntComp::build_from_stdin();
    let panels = run_robot(&ic, 0);
    println!("{}", panels.keys().len());
    let panels = run_robot(&ic, 1);
    let minx = panels.keys().map(|x| x.0).min().unwrap();
    let miny = panels.keys().map(|x| x.1).min().unwrap();
    let maxx = panels.keys().map(|x| x.0).max().unwrap();
    let maxy = panels.keys().map(|x| x.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            let v = *panels.get(&(x, y)).or(Some(&0)).unwrap();
            print!(
                "{}",
                match v {
                    1 => "#",
                    _ => " ",
                }
            );
        }
        println!();
    }
}
