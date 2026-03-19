use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    for _ in 0..7 {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        // The specification states overflow behavior is not inferable/undefined.
        // Use normal subtraction; in debug builds Rust may panic on overflow,
        // which is acceptable given the precondition that results are representable.
        let diff = a - b;
        println!("{diff}");
    }
}
