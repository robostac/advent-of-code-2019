use intcomp::IntComp;
use std::collections::HashSet;

fn part1(ic: &IntComp, allow_nat: bool) -> isize {
    let mut network = Vec::new();
    let mut natx = 0;
    let mut naty = 0;
    let mut seeny = HashSet::new();
    for x in 0..50 {
        network.push(ic.clone());
        network[x].push_input(x as isize);
    }
    loop {
        let mut idle = true;
        for i in 0..50 {
            network[i].run_till_input();
            if network[i].output_len() >= 3 {
                let a = network[i].pop_output() as usize;
                let x = network[i].pop_output();
                let y = network[i].pop_output();
                if a == 255 {
                    if allow_nat == false {
                        return y;
                    }
                    natx = x;
                    naty = y
                } else {
                    network[a].push_input(x);
                    network[a].push_input(y);
                }
                idle = false;
            }
            if network[i].needs_input() {
                network[i].push_input(-1);
            }
        }
        if idle {
            if seeny.insert(naty) == false {
                break;
            }
            network[0].push_input(natx);
            network[0].push_input(naty);
        }
    }
    naty
}

fn main() {
    let ic = IntComp::build_from_stdin();
    println!("{}", part1(&ic, false));
    println!("{}", part1(&ic, true));
}
