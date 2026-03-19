use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Store up to 100 elements (spec bound). C code uses 1-based indexing.
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
            i += 2; // skip next index to avoid overlap (matches C's i++ inside loop)
        } else {
            i += 1;
        }
    }

    print!("{count}\n");
}
