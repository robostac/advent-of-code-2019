// cargo-deps: text_io
use intcomp::IntComp;
use std::collections::HashMap;
use std::collections::HashSet;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut tilesic = ic.clone();
    let mut grid = HashMap::new();
    tilesic.run();
    let mut x = 0;
    let mut y = 0;
    for v in tilesic.output_ref() {
        if *v == 10 {
            y += 1;
            x = 0;
        } else {
            grid.insert((x, y), *v);
            x += 1;
        }
    }
    let intersection_points: isize = grid
        .iter()
        .filter_map(|(k, v)| {
            for d in DIRECTIONS.iter() {
                let np = (k.0 + d.0, k.1 + d.1);
                if *grid.get(&np).or(Some(&0)).unwrap() != 35 {
                    return None;
                }
            }
            if *v != 35 {
                return None;
            }
            Some(k.0 * k.1)
        })
        .sum();
    draw(&grid);
    println!("{:?}", intersection_points);

    let start = grid.iter().find(|(_k, v)| **v == '^' as isize).unwrap().0;
    let v = dfs(&grid, &Vec::new(), &Vec::new(), *start, (0, -1)).unwrap();

    let route = (v.1)
        .iter()
        .map(|x| format!("{}", (*x as u8 + 'A' as u8) as char))
        .collect::<Vec<String>>()
        .join(",");
    let mut tilesic = ic.clone();
    *tilesic.values.entry(0).or_insert(0) = 2;
    for x in route.bytes() {
        tilesic.push_input(x as isize);
    }
    tilesic.push_input('\n' as isize);
    for p in (v.0).iter() {
        let prog = p.join(",");
        for x in prog.bytes() {
            tilesic.push_input(x as isize);
        }
        tilesic.push_input('\n' as isize);
    }
    tilesic.push_input('n' as isize);
    tilesic.push_input('\n' as isize);
    tilesic.run();
    println!("{:?}", tilesic.output_ref()[tilesic.output_len() - 1]);
}

fn apply_prog(
    gamestate: &HashMap<(isize, isize), isize>,
    mut pos: (isize, isize),
    mut dir: (isize, isize),
    prog: &Vec<String>,
    debug: bool,
    visit: Option<&mut HashSet<(isize, isize)>>,
) -> Option<((isize, isize), (isize, isize))> {
    let mut hs = HashSet::new();
    let visit = visit.or(Some(&mut hs)).unwrap();
    for p in prog.iter() {
        visit.insert(pos);
        if debug {
            println!("{:?} {:?} {:?}", pos, dir, p);
        }
        match p.as_ref() {
            "R" => dir = (-dir.1, dir.0),
            "L" => dir = (dir.1, -dir.0),
            v => {
                let dist = v.parse::<isize>().unwrap();
                for _i in 0..dist {
                    visit.insert(pos);
                    let np = (pos.0 + dir.0, pos.1 + dir.1);

                    if np.0 == 0 {
                        pos = np;
                        break;
                    }
                    if *gamestate.get(&np).or(Some(&46)).unwrap() == 46 {
                        return None;
                    } else {
                        pos = np;
                    }
                }
            }
        }
    }
    Some((pos, dir))
}

fn get_moves(
    gamestate: &HashMap<(isize, isize), isize>,
    mut pos: (isize, isize),
    dir: (isize, isize),
) -> Vec<String> {
    let mut s = Vec::new();
    let left = (dir.1, -dir.0);
    let right = (-dir.1, dir.0);
    let mut count = 0;
    loop {
        if count == 0 {
            let npl = (pos.0 + left.0, pos.1 + left.1);
            if *gamestate.get(&npl).or(Some(&46)).unwrap() != 46 {
                if count == 0 {
                    s.push("L".to_owned());
                } else {
                    s.push(format!("{}", count));
                }
            }
            let npr = (pos.0 + right.0, pos.1 + right.1);
            if *gamestate.get(&npr).or(Some(&46)).unwrap() != 46 {
                if count == 0 {
                    s.push("R".to_owned());
                } else {
                    s.push(format!("{}", count));
                }
            }
        }
        let np = (pos.0 + dir.0, pos.1 + dir.1);
        if *gamestate.get(&np).or(Some(&46)).unwrap() == 46 {
            break;
        }
        pos = np;
        count += 1;
    }
    if count > 0 {
        s.push(format!("{}", count));
    }
    s
}

fn generate_programs(
    gamestate: &HashMap<(isize, isize), isize>,
    pos: (isize, isize),
    dir: (isize, isize),
    progs: &mut Vec<Vec<String>>,
    cur_prog: &Vec<String>,
) {
    let moves = get_moves(gamestate, pos, dir);
    let s = cur_prog.join(",");
    if s.len() > 20 {
        return;
    }
    let test = "".to_owned();
    let v = cur_prog.get(cur_prog.len() - 1).or(Some(&test)).unwrap();
    let last_was_dir = v == "L" || v == "R";
    for m in moves.iter() {
        let this_was_dir = m == "L" || m == "R";
        if this_was_dir == last_was_dir {
            continue;
        }

        let x = apply_prog(gamestate, pos, dir, &vec![m.to_owned()], false, None);
        if x != None {
            let mut cur_prog = cur_prog.clone();
            cur_prog.push(m.to_owned());
            let (np, nd) = x.unwrap();
            if m != "L" && m != "R" {
                progs.push(cur_prog.clone());
            }
            generate_programs(gamestate, np, nd, progs, &cur_prog);
        }
    }
}

fn dfs(
    gamestate: &HashMap<(isize, isize), isize>,
    prog: &Vec<Vec<String>>,
    route: &Vec<usize>,
    pos: (isize, isize),
    dir: (isize, isize),
) -> Option<(Vec<Vec<String>>, Vec<usize>)> {
    // println!("{:?} {:?} {:?} {:?}", route, pos, dir, prog);
    //add to current_prog
    if pos.0 == 0 {
        let mut start = *gamestate
            .iter()
            .find(|(_k, v)| **v == '^' as isize)
            .unwrap()
            .0;
        let visits = gamestate
            .iter()
            .filter(|(_k, v)| **v == '#' as isize)
            .count();
        let mut start_dir = (0, -1);
        let mut hs = HashSet::new();
        for x in route.iter() {
            let (np, nd) =
                apply_prog(gamestate, start, start_dir, &prog[*x], false, Some(&mut hs)).unwrap();
            start = np;
            start_dir = nd;
        }
        if hs.len() == visits {
            return Some((prog.clone(), route.clone()));
        }
    }
    //try to apply other prog
    for (i, p) in prog.iter().enumerate() {
        let v = apply_prog(gamestate, pos, dir, p, false, None);
        let mut r = route.clone();
        r.push(i);
        if r.len() > 10 {
            continue;
        }
        let x = match v {
            Some((np, nd)) => dfs(gamestate, prog, &r, np, nd),
            _ => None,
        };
        if x != None {
            //     println!("{:?} {:?} {:?} {:?}", pos, dir, p, v);
            return x;
        }
    }
    if prog.len() < 3 {
        let mut pp = Vec::new();
        generate_programs(gamestate, pos, dir, &mut pp, &Vec::new());
        pp.sort_by_key(|x| -(x.len() as isize));
        // println!("{:?} {:?} {:?}", pos, dir, pp);
        for p in pp.iter() {
            let v = apply_prog(gamestate, pos, dir, p, false, None);
            if v != None {
                let (np, nd) = v.unwrap();
                let mut prog = prog.clone();
                prog.push(p.clone());
                let mut r = route.clone();
                r.push(prog.len() - 1);
                let v = dfs(gamestate, &prog, &r, np, nd);
                if v != None {
                    return v;
                }
            }
        }
    }

    //new prog

    None
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
