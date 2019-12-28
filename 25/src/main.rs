// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
struct Room {
    name: String,
    directions: [Option<String>; 4],
    items: Vec<String>,
}

fn parse_output(s: &String) -> Option<Room> {
    let c: Vec<String> = s.split('\n').map(|x| x.to_owned()).collect();
    let name = c
        .iter()
        .find(|x| x.chars().next().or(Some(' ')).unwrap() == '=');
    if name != None {
        let mut it_vec = Vec::new();
        let mut dir_vec = [None, None, None, None];
        let name = name.unwrap().to_owned();
        let mut directions = false;
        let mut items = false;
        for x in c.iter() {
            if x == "Items here:" {
                items = true;
            } else if x == "Doors here lead:" {
                directions = true;
            } else if x == "" {
                directions = false;
                items = false;
            } else if directions {
                let z = &x[2..];
                dir_vec[DIRECTIONS_STR.iter().position(|&r| r == z).unwrap()] = Some("".to_owned());
            } else if items {
                let z = &x[2..];
                it_vec.push(z.to_owned())
            }
        }
        Some(Room {
            name: name,
            directions: dir_vec,
            items: it_vec,
        })
    } else {
        None
    }
}

const DIRECTIONS_STR: [&str; 4] = ["north", "south", "west", "east"];
fn bfs(grid: &HashMap<String, Room>, loc: &String, target: Option<&String>) -> isize {
    let mut seen = HashSet::new();
    let mut count = 0;
    let mut cur = vec![(loc.to_owned(), -1)];
    while cur.is_empty() == false {
        count += 1;
        let mut next = Vec::new();
        for x in cur {
            let dir = grid[&x.0].directions.clone();
            for (i, z) in dir.iter().enumerate() {
                if *z == None {
                    continue;
                }
                let z = z.as_ref().unwrap().to_owned();
                if x.0 == "== Security Checkpoint ==" && i == 0 {
                    continue;
                }
                let dir = if x.1 >= 0 { x.1 } else { i as isize };
                if z == "" {
                    if target == None {
                        return dir;
                    }
                    continue;
                }
                if target != None && z == *target.unwrap() {
                    return dir;
                }
                if seen.insert(z.to_owned()) {
                    next.push((z, dir));
                }
            }
        }
        cur = next;
    }
    -1
}

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut tilesic = ic.clone();
    let mut undo = Vec::new();
    let mut state = 0;
    let mut rooms = HashMap::new();
    let mut prev = "".to_owned();
    let mut last_move: isize = -1;
    let mut items = Vec::new();
    let mut mv = -1;
    let mut cv = 0;
    while tilesic.finished() == false {
        tilesic.run_till_input();
        undo.push(tilesic.clone());
        let mut v = Vec::new();
        while tilesic.output_len() > 0 {
            v.push(format!("{}", tilesic.pop_output() as u8 as char));
        }
        let output = v.join("");
        println!("{}", output);
        let test = parse_output(&output);
        if state == 0 {
            let test = test.unwrap();
            if rooms.contains_key(&test.name) == false {
                rooms.insert(test.name.to_owned(), test.clone());
                if last_move != -1 {
                    let k = rooms.entry(prev.to_owned()).or_default();
                    k.directions[last_move as usize] = Some(test.name.to_owned());
                    let k = rooms.entry(test.name.to_owned()).or_default();
                    k.directions[(last_move ^ 1) as usize] = Some(prev.to_owned());
                }
            }
            prev = test.name.to_owned();
            let mut dir = bfs(&rooms, &prev, None);
            for x in test.items {
                if x == "infinite loop" {
                    continue;
                }
                if x == "photons" {
                    continue;
                }
                if x == "giant electromagnet" {
                    continue;
                }
                if x == "molten lava" {
                    continue;
                }
                if x == "escape pod" {
                    continue;
                }
                let out = format!("take {}", x);
                items.push(x.to_owned());
                tilesic.push_input_string(&out);
            }
            if dir < 0 {
                let tgt = "== Security Checkpoint ==".to_owned();
                if test.name == tgt {
                    state = 1
                } else {
                    dir = bfs(&rooms, &prev, Some(&tgt));
                    println!("{}", dir);
                }
            }
            if dir >= 0 {
                last_move = dir;
                tilesic.push_input_string(&DIRECTIONS_STR[dir as usize]);
            }
        } else if state == 1 {
            if mv < 0 {
                mv = 1 << items.len();
            }
            for x in 0..items.len() {
                let out;
                if (cv & (1 << x)) > 0 {
                    out = format!("take {}", items[x]);
                } else {
                    out = format!("drop {}", items[x]);
                }

                tilesic.push_input_string(&out);
            }
            let out = "north";
            tilesic.push_input_string(&out);
            cv += 1;
        }
    }
}
