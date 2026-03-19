use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // If fewer than three integers are present, behavior is unspecified by the spec.
    // We choose to produce no output and exit successfully.
    let _first: i128 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let second: i128 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let third: i128 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let diff = third - second;
    if diff % 2 == 0 {
        println!("Alice");
    } else {
        println!("Borys");
    }
}
