use std::io::{self, Read};

const N: i64 = 8;
const MD: i64 = 1_000_000_007;

fn min_i64(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}
fn max_i64(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn power_mod(mut a: i64, mut k: i64) -> i64 {
    // Computes a^k mod MD using fast exponentiation.
    a %= MD;
    let mut p: i64 = 1;
    let mut b: i64 = a;
    while k != 0 {
        if (k & 1) != 0 {
            p = (p * b) % MD;
        }
        b = (b * b) % MD;
        k >>= 1;
    }
    p
}

fn main() {
    // Read exactly one integer s.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Build cnt[1..=8]: cnt[1]=9, cnt[d]=10*cnt[d-1]
    // Note: per spec/C, this is 9*10^(d-1), i.e., the maximum d-digit positive integer.
    let mut cnt = vec![0i64; (N as usize) + 1];
    cnt[1] = 9;
    for i in 2..=N as usize {
        cnt[i] = cnt[i - 1] * 10;
    }

    // ans initialized to (s-1)/8 (C integer division; for positive s this is floor).
    let mut ans: i64 = (s - 1) / N;

    for l1 in 1..=N {
        // A) Single-length divisibility contribution
        if s % l1 == 0 {
            let q = s / l1;
            if cnt[l1 as usize] >= q {
                ans = (ans + (cnt[l1 as usize] - q + 1)) % MD;
            }
        }

        // B) Two-length mixed contribution with progressive reduction
        let mut s_ = s;
        for l2 in (l1 + 1)..=N {
            let lower = (max_i64(s_ - cnt[l2 as usize] * l2, 1) + l1 - 1) / l1;
            let upper = min_i64((s_ - l2) / l1, cnt[l1 as usize]);

            if lower <= upper {
                for r1 in 1..=l2 {
                    if (s_ - r1 * l1) % l2 == 0 {
                        let add = (upper + l2 - r1) / l2 - (lower - 1 + l2 - r1) / l2;
                        ans = (ans + add) % MD;
                    }
                }
            }

            if s_ / l2 < cnt[l2 as usize] {
                break;
            }
            s_ -= cnt[l2 as usize] * l2;
        }
    }

    // Additional contributions for digit lengths > 8 via divisors
    // Iterate a from 1 to floor(sqrt(s)) using condition a <= s/a (as in C).
    let mut a: i64 = 1;
    while a <= s / a {
        if s % a == 0 {
            let b = s / a;
            if a > N {
                ans = (ans + (power_mod(10, a - 1) * 9 - b)) % MD;
            }
            if a != b && b > N {
                ans = (ans + (power_mod(10, b - 1) * 9 - a)) % MD;
            }
        }
        a += 1;
    }

    // Final normalization: only one +MD if negative (matches C behavior).
    if ans < 0 {
        ans += MD;
    }

    print!("{ans}\n");
}
