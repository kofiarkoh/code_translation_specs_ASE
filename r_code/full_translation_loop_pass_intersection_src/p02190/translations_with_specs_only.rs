use std::io::{self, Read};

fn main() {
    // No explicit error handling per specification; unwraps may panic on malformed input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed maximum capacity per specification.
    // If preconditions are violated, behavior is unspecified; we do not add explicit checks.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    a.sort();

    // Preconditions guarantee n >= 1, so the distinct count starts at 1.
    let mut distinct: usize = 1;
    for i in 1..n {
        if a[i] != a[i - 1] {
            distinct += 1;
        }
    }

    print!("{}", distinct);
}
