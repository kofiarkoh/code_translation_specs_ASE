use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // Unspecified behavior; exit quietly if no input.
    };

    // Fixed storage up to 100 elements as per specification.
    // If n > 100, behavior is unspecified; we still attempt to read n values.
    let mut a = [0i64; 100];
    for i in 0..n.min(100) {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    // If n > 100, consume remaining inputs (unspecified; keep minimal behavior).
    for _ in 100..n {
        let _ = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut count: i64 = 0;
    let mut i: usize = 0;

    // Count non-overlapping adjacent equal pairs; each counted pair consumes both elements.
    while i + 1 < n {
        let x = if i < 100 { a[i] } else { 0 }; // Unspecified for n>100; keep defined.
        let y = if i + 1 < 100 { a[i + 1] } else { 0 };
        if x == y {
            count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    print!("{}", count);
}
