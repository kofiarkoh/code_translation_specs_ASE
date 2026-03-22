use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed maximum capacity per spec: 100001.
    // Preconditions guarantee n <= 100001 and n >= 1.
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Sort ascending (matches C qsort with comparator).
    a.sort_unstable();

    // Count distinct values in sorted array.
    let mut ret: i32 = 1;
    let mut prev = a[0];
    for &x in a.iter().skip(1) {
        if x != prev {
            ret += 1;
        }
        prev = x;
    }

    print!("{ret}\n");
}
