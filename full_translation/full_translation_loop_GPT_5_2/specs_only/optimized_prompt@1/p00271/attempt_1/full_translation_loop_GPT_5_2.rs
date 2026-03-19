use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is undefined if input is not exactly 14 integers as specified.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();
    for i in 0..7 {
        let a: i128 = it.next().unwrap().parse().unwrap();
        let b: i128 = it.next().unwrap().parse().unwrap();
        let diff = a - b;
        out.push_str(&diff.to_string());
        if i != 6 {
            out.push('\n');
        }
    }

    print!("{out}");
}
