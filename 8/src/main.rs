use std::io;
fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let width = 25;
    let height = 6;
    let layers = s.len() / (width * height);
    let values: Vec<usize> = s
        .as_bytes()
        .iter()
        .map(|&x| x as usize - '0' as usize)
        .collect();
    let mv = values
        .chunks(width * height)
        .min_by_key(|chunk| chunk.iter().filter(|&x| *x == 0).count())
        .unwrap();
    println!(
        "{:?}",
        mv.iter().filter(|&x| *x == 1).count() * mv.iter().filter(|&x| *x == 2).count()
    );
    for y in 0..height {
        for x in 0..width {
            let idx = x + y * width;
            for l in 0..layers {
                match values[idx + l * width * height] {
                    1 => {
                        print!("#");
                        break;
                    }
                    0 => {
                        print!(" ");
                        break;
                    }
                    _ => (),
                }
            }
        }
        println!();
    }
}
