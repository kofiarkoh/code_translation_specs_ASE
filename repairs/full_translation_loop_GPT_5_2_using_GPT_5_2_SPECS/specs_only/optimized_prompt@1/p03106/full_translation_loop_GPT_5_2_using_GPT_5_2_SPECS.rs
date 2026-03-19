use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse three integers.
    // Specification says behavior is not well-defined on missing/invalid input;
    // we keep parsing minimal and will panic if parsing fails.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let c: i64 = it.next().unwrap().parse().unwrap();

    // Specification: time proportional to min(a, b) for valid positive inputs,
    // constant extra memory. We therefore scan all candidates from min(a,b) down to 1.
    // For invalid preconditions, output is not well-defined; we do not add checks.
    let mut remaining = c;
    let limit = if a < b { a } else { b };

    let mut ans: i64 = 0; // Placeholder; under valid preconditions it will be set.
    let mut i = limit;
    while i >= 1 {
        if a % i == 0 && b % i == 0 {
            remaining -= 1;
            if remaining == 0 {
                ans = i;
                break;
            }
        }
        i -= 1;
    }

    println!("{}", ans);
}
