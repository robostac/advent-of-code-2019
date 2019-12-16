use intcomp::IntComp;

fn main() {
    let ic = IntComp::build_from_stdin();
    let mut tempic = ic.clone();
    tempic.push_input(1);
    tempic.run();
    println!("{:?}", tempic.pop_output());
    let mut tempic = ic.clone();
    tempic.push_input(2);
    tempic.run();
    println!("{:?}", tempic.pop_output());
}
