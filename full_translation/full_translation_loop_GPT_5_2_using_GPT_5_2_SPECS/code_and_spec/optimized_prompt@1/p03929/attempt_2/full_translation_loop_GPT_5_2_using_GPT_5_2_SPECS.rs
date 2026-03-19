use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n_in: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Per spec: if k not in [0,10], no solution for (9i) mod 11 == k.
    if !(0..=10).contains(&k) {
        print!("0\n");
        return;
    }

    // Compute N = 7n (spec assumes it fits in i64; avoid panic on overflow).
    let n = match n_in.checked_mul(7) {
        Some(v) => v,
        None => {
            // Overflow is undefined per spec; choose a benign output.
            print!("0\n");
            return;
        }
    };

    if n < 8 {
        print!("0\n");
        return;
    }

    // Follow the C program's exact counting strategy (period 693) to match intended behavior.
    // Note: This differs from the earlier 77-period approach; the C code uses 693 and then
    // subtracts boundary contributions to effectively count i in [8, n].
    let mut ans: i64 = 0;

    // First partial block: i = 1..=n%693
    let r = n.rem_euclid(693);
    for i in 1..=r {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans += 1;
        }
    }

    // Full blocks count over 1..=693
    let mut count: i64 = 0;
    for i in 1..=693 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            count += 1;
        }
    }
    ans += count * (n / 693);

    // Subtract i in 1..=7 (exclude below 8)
    for i in 1..=7 {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans -= 1;
        }
    }

    // Subtract i in [n-6, n] (C code does this; keep identical behavior).
    // Use a safe range even if n < 6 (though n>=8 here).
    let start = n - 6;
    for i in start..=n {
        if i % 7 > 1 && (9 * i) % 11 == k {
            ans -= 1;
        }
    }

    print!("{ans}\n");
}
