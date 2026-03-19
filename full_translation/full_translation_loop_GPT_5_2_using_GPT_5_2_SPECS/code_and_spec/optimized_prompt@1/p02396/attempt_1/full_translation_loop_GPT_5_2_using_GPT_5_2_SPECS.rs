use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior on invalid/insufficient input is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let mut i: i32 = 0;

    loop {
        // No explicit handling for end-of-input/read failures per spec.
        let x_str = iter.next().unwrap();
        let x: i32 = x_str.parse().unwrap();

        i += 1;
        if x == 0 {
            break;
        }
        print!("Case {}: {}\n", i, x);
    }
}
