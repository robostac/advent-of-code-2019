use std::io;
use std::io::prelude::*;

fn fft(inp: &Vec<i64>) -> Vec<i64> {
    let mut outp = Vec::new();
    outp.reserve(inp.len());
    for i in 0..inp.len() {
        let total: i64 = inp[i..]
            .chunks(i + 1)
            .step_by(4)
            .map(|x| {
                let v: i64 = x.iter().sum();
                v
            })
            .sum();
        let minus: i64 = inp[i..]
            .chunks(i + 1)
            .skip(2)
            .step_by(4)
            .map(|x| {
                let v: i64 = x.iter().sum();
                v
            })
            .sum();
        let val = total - minus;
        outp.push(val.abs() % 10);
    }
    outp
}

fn main() {
    let stdin = io::stdin();
    let valuesorig: Vec<i64> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();
    let phases = 100;
    let mut values = valuesorig.clone();
    for _i in 0..phases {
        values = fft(&values);
    }
    println!("{:?}", &values[0..8]);
    let mut values = valuesorig.clone();
    for _i in 0..9999 {
        values.extend(valuesorig.iter());
    }

    let mut offset = 0;
    let mut mult = 1000000;
    for x in valuesorig[0..7].iter() {
        offset += (*x * mult) as usize;
        mult /= 10;
    }
    if offset < values.len() / 2 {
        panic!("Part 2 only works if offset is after halfway")
    }
    for _i in 0..phases {
        let mut total = 0;
        for v in values[offset..].iter_mut().rev() {
            total += *v;
            *v = total % 10;
        }
    }
    println!("{:?}", &values[offset..offset + 8]);
}
