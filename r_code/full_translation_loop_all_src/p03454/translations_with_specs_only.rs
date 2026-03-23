use std::io::{self, Read};

const N: i64 = 8;
const MD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64, modu: i64) -> i64 {
    // Fast exponentiation modulo modu; exp is assumed >= 0 in all specified uses.
    let mut res: i64 = 1 % modu;
    base %= modu;
    while exp > 0 {
        if (exp & 1) != 0 {
            res = (res * base) % modu;
        }
        base = (base * base) % modu;
        exp >>= 1;
    }
    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // cnt[d] for d=1..8, with cnt[d] = 10^d - 1 (also max d-digit positive integer).
    let mut cnt = vec![0i64; (N as usize) + 1];
    cnt[1] = 9;
    for d in 2..=N as usize {
        cnt[d] = 10 * cnt[d - 1];
    }

    // Initialization term: (s - 1) / 8, truncating toward zero (Rust matches C for i64).
    let mut ans: i64 = (s - 1) / N;

    // Contributions for digit lengths up to 8.
    for l1 in 1..=N {
        // A) Single-length divisibility contribution.
        if l1 != 0 && s % l1 == 0 {
            let q = s / l1;
            if q <= cnt[l1 as usize] {
                ans = (ans + (cnt[l1 as usize] - q + 1)) % MD;
            }
        }

        // B) Two-length mixed contribution with progressive reduction.
        let mut s_ = s;
        for l2 in (l1 + 1)..=N {
            // lower = ceil(max(s_ - cnt[l2]*l2, 1) / l1)
            let t = s_ - cnt[l2 as usize] * l2;
            let m = if t > 1 { t } else { 1 };
            // l1 is positive, so (m + l1 - 1) / l1 is the intended ceil division.
            let lower = (m + l1 - 1) / l1;

            // upper = min(floor((s_ - l2) / l1), cnt[l1])
            let upper_raw = (s_ - l2) / l1;
            let upper = if upper_raw < cnt[l1 as usize] {
                upper_raw
            } else {
                cnt[l1 as usize]
            };

            if lower <= upper {
                for r1 in 1..=l2 {
                    if (s_ - r1 * l1) % l2 == 0 {
                        // count = floor((upper + l2 - r1)/l2) - floor((lower - 1 + l2 - r1)/l2)
                        let c1 = (upper + l2 - r1) / l2;
                        let c0 = (lower - 1 + l2 - r1) / l2;
                        let count = c1 - c0;
                        ans = (ans + count) % MD;
                    }
                }
            }

            // Early termination / state update for s_.
            if (s_ / l2) < cnt[l2 as usize] {
                break;
            } else {
                s_ = s_ - cnt[l2 as usize] * l2;
            }
        }
    }

    // Additional contributions for digit lengths greater than 8 via divisors.
    // Iterate a from 1 while a <= s / a (i.e., up to floor(sqrt(s))).
    let mut a: i64 = 1;
    while a > 0 && a <= s / a {
        if s % a == 0 {
            let b = s / a;

            if a > N {
                // Add (9 * 10^(a-1) - b) modulo MD.
                let term = (9i64 * mod_pow(10, a - 1, MD) - b) % MD;
                ans = (ans + term) % MD;
            }
            if a != b && b > N {
                // Add (9 * 10^(b-1) - a) modulo MD.
                let term = (9i64 * mod_pow(10, b - 1, MD) - a) % MD;
                ans = (ans + term) % MD;
            }
        }
        a += 1;
    }

    // Final normalization: if ans < 0, add MD once.
    if ans < 0 {
        ans += MD;
    }

    println!("{ans}");
}
