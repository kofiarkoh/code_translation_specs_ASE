use std::io::{self, Read};

fn f(level: i64, pos: i64) -> i64 {
    if level == 0 {
        if pos <= 0 { 0 } else { 1 }
    } else {
        // Preconditions state (2^level - 1) is representable; overflow behavior is unspecified.
        // Use i64 arithmetic directly to match that intent.
        let p: i64 = (1_i64 << (level as u32)) - 1;
        if pos <= 2 * p {
            f(level - 1, pos - 1)
        } else {
            (p + 1) + f(level - 1, pos - 1 - 2 * p)
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let ans = f(n, k);
    print!("{}", ans);
}
