use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
fn count(x: &String, dict: &HashMap<String, Vec<String>>) -> usize {
    match dict.get(x) {
        Some(s) => s.iter().map(|x| count(x, &dict) + 1).sum(),
        None => 0,
    }
}

fn bfs(start: String, tgt: String, dict: &HashMap<String, Vec<String>>) -> usize {
    let mut current: HashSet<&String> = [&start].iter().cloned().collect();
    let mut seen = HashSet::<&String>::new();
    for inc in 0..dict.keys().count() {
        current = current
            .iter()
            .map(|x| dict[*x].iter().collect())
            .collect::<Vec<Vec<&String>>>()
            .concat()
            .iter()
            .cloned()
            .collect();
        current = current.difference(&seen).cloned().collect();
        seen.extend(current.iter().cloned());
        if seen.contains(&tgt) {
            return inc - 1;
        }
    }
    0
}

fn main() {
    let stdin = io::stdin();
    let mut orbits = HashMap::new();
    let mut links = HashMap::new();
    for x in stdin.lock().lines() {
        let z = x.unwrap();
        let s: Vec<&str> = z.split(")").collect();
        orbits
            .entry(s[0].to_owned())
            .or_insert(Vec::<String>::new())
            .push(s[1].to_owned());
        links
            .entry(s[1].to_owned())
            .or_insert(Vec::<String>::new())
            .push(s[0].to_owned());
        links
            .entry(s[0].to_owned())
            .or_insert(Vec::<String>::new())
            .push(s[1].to_owned());
    }
    let v: usize = orbits.keys().map(|x| count(x, &orbits)).sum();
    println!("{}", v);
    println!("{:?}", bfs("YOU".to_owned(), "SAN".to_owned(), &links));
}
