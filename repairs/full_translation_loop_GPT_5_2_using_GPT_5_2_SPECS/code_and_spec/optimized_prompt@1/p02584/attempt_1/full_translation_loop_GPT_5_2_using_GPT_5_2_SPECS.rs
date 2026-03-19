use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();
    let d_in: i64 = it.next().unwrap().parse().unwrap();

    // SPEC: use step magnitude (absolute value)
    let d: i64 = d_in.abs();
    let x_abs: i64 = x.abs();

    let q = x_abs / d; // floor(X / D)
    let ans: i64;

    if k <= q {
        // |X - K*D|
        ans = (x_abs - k * d).abs();
    } else {
        // R if (K - q) even else |R - D|
        let r = x_abs % d;
        let k_prime = k - q;
        if k_prime % 2 == 0 {
            ans = r;
        } else {
            ans = (r - d).abs();
        }
    }

    print!("{}", ans);
}
