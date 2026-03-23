use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf-style whitespace-separated parsing).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Unspecified behavior in C if missing; just exit.
    };

    // 1-based indexing to mirror the C program.
    let mut a = vec![0i64; n + 1];
    for i in 1..=n {
        if let Some(v) = it.next() {
            a[i] = v.parse::<i64>().unwrap();
        } else {
            // Unspecified behavior in C if insufficient input; stop.
            return;
        }
    }

    let mut ans: i64 = 0;

    for i in 1..n {
        ans += a[i] / 2;
        if a[i + 1] > 1 && (a[i] % 2 == 1) {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n] / 2;

    // Print exactly one integer followed by newline.
    println!("{ans}");
}
