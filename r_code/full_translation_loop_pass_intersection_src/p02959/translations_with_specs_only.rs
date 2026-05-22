use std::io::{self, Read};

fn main() {
    // Read all input; malformed/insufficient input => unspecified behavior per spec.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Spec implies fixed storage bounds in the original; here we allocate dynamically.
    // Preconditions guarantee n <= 199999 and values fit in signed 64-bit.
    let mut a: Vec<i64> = Vec::with_capacity(n + 1);
    for _ in 0..(n + 1) {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut total: i64 = 0;

    for i in 0..n {
        let b: i64 = it.next().unwrap().parse().unwrap();

        // Consume from A_i first.
        let take_from_current = if a[i] < b { a[i] } else { b };
        a[i] -= take_from_current;
        let mut rem = b - take_from_current;

        // Then consume remaining from A_{i+1}.
        let take_from_next = if a[i + 1] < rem { a[i + 1] } else { rem };
        a[i + 1] -= take_from_next;

        // Accumulate total consumed for this demand.
        // Note: In the original C-like spec, signed overflow would be UB; Rust would panic in debug.
        // We keep normal i64 arithmetic to match the intended contract under the precondition
        // that overflow does not occur.
        total += take_from_current + take_from_next;
    }

    println!("{}", total);
}
