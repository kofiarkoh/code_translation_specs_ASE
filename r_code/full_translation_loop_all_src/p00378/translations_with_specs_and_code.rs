use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let x: i64 = it.next().unwrap().parse().unwrap();

    // Semantics follow the provided C code (spec does not define the exact formula).
    let ans: i64;
    if a >= 2 * b {
        // ((X - 1) / 500 + 1) * B
        ans = ((x - 1) / 500 + 1) * b;
    } else {
        // X / 1000 * A, then handle remainder
        let mut tmp = (x / 1000) * a;
        let c = x % 1000;
        if c != 0 {
            if c <= 500 && b < a {
                tmp += b;
            } else {
                tmp += a;
            }
        }
        ans = tmp;
    }

    print!("{ans}\n");
}
