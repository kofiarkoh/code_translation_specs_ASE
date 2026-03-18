use std::io::{self, Read};

type LL = i64;

fn f(n: LL, x: LL) -> LL {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }

    // C uses (1LL<<n) which is undefined for n >= 63 on 64-bit long long.
    // The spec only states non-negative inputs; to preserve intended behavior
    // for typical valid ranges, we compute with checked shift and saturate.
    let p: LL = match 1_i64.checked_shl(n as u32) {
        Some(v) => v - 1,
        None => i64::MAX, // saturate to avoid panic/UB; keeps result non-negative
    };

    if x <= 2 * p {
        f(n - 1, x - 1)
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: LL = it.next().unwrap().parse().unwrap();
    let k: LL = it.next().unwrap().parse().unwrap();

    let ans = f(n, k);
    print!("{ans}");
}
