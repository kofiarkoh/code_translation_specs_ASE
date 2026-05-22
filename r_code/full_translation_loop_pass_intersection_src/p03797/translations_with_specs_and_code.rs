use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed integers (behavior unspecified if missing/invalid).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // Per spec: h = trunc(m/2). Rust integer division truncates toward zero.
    let h: i64 = m / 2;

    let sum: i64 = if m == 1 {
        0
    } else if h < n {
        h
    } else if 0 <= (h - n) && (h - n) < 2 {
        n
    } else {
        // h - n >= 2
        n + (h - n) / 2
    };

    // Preserve exact I/O formatting: print one integer, no trailing newline.
    print!("{sum}");
}
