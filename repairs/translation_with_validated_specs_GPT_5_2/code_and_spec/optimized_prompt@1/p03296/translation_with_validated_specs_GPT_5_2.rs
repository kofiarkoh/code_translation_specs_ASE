use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // unspecified behavior if missing; just exit
    };

    // Fixed storage up to 100 elements (spec). C uses 1-based indexing with a[101].
    // We'll store 1..=n in a vector of length n+1.
    let mut a: Vec<i32> = vec![0; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // Count non-overlapping adjacent equal pairs.
    let mut count: i32 = 0;
    let mut i: usize = 2;
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 2; // consume both elements; pairs do not overlap
        } else {
            i += 1;
        }
    }

    // Preserve output formatting: single integer followed by newline.
    print!("{count}\n");
}
