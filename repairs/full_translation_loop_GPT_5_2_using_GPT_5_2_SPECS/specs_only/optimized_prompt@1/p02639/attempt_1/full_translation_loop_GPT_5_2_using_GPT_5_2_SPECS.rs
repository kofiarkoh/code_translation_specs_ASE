use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Specification precondition: at least five integer values are provided.
    // If fewer than five are present or parsing fails, behavior is unspecified by the spec;
    // we choose to do nothing in that case (narrow, non-inventive behavior).
    let mut vals = [0i64; 5];
    for i in 0..5 {
        match it.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => vals[i] = v,
            None => return,
        }
    }

    for (idx, &v) in vals.iter().enumerate() {
        if v == 0 {
            print!("{}", idx + 1);
            return;
        }
    }
}
