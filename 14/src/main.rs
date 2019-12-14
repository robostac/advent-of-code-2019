// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
#[derive(Debug, Clone)]
struct Reaction {
    output: String,
    count: i64,
    requires: Vec<(String, i64)>,
}

fn parse_reaction(s: &str) -> Reaction {
    let name: String;
    let count: i64;
    let req_arr: String;
    scan!(s.bytes() => "{}=> {} {}", req_arr, count, name);
    let req: Vec<(String, i64)> = req_arr
        .split(", ")
        .map(|x| {
            let v: i64;
            let name: String;
            scan!(x.bytes() => "{} {}", v, name);
            (name, v)
        })
        .collect();
    Reaction {
        output: name,
        count: count,
        requires: req,
    }
}

fn calc_ore_req(values: &Vec<Reaction>, required: i64) -> i64 {
    let mut spare: HashMap<String, i64> = HashMap::new();
    let mut used = 0;
    let mut requirements: VecDeque<(String, i64)> =
        [("FUEL".to_owned(), required)].iter().cloned().collect();
    while requirements.is_empty() != true {
        let next = requirements.pop_front().unwrap();
        if next.0 == "ORE" {
            used += next.1;
            continue;
        }
        let sc = spare.entry(next.0.clone()).or_insert(0);
        let req = std::cmp::max(next.1 - *sc, 0);
        *sc -= next.1 - req;
        if req == 0 {
            continue;
        }
        let recipe = values.iter().find(|&x| x.output == next.0).unwrap();
        let mut counts = req / recipe.count;
        if (req % recipe.count) != 0 {
            counts += 1;
        }
        *sc += (counts * recipe.count) - req;
        for x in recipe.requires.iter() {
            requirements.push_back((x.0.clone(), x.1 * counts));
        }
    }
    used
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Reaction> = stdin
        .lock()
        .lines()
        .map(|input| parse_reaction(&input.unwrap()))
        .collect();
    let fuel_for_one = calc_ore_req(&values, 1);
    println!("{:?}", fuel_for_one);
    let max_ore = 1000000000000;
    let mut min_fuel = 1;
    let mut max_fuel = max_ore;
    while min_fuel + 1 < max_fuel {
        let pivot = (max_fuel + min_fuel) / 2;
        let req = calc_ore_req(&values, pivot);
        if req > max_ore {
            max_fuel = pivot
        } else {
            min_fuel = pivot;
        }
    }
    println!("{}", min_fuel,);
}
