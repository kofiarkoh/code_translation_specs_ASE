use std::io::{self, Read};

const N: usize = 8;
const MD: i64 = 1_000_000_007;

fn min_i64(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}
fn max_i64(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn init_cnt() -> [i64; N + 1] {
    // cnt[i] in C is int; values up to 9*10^7 fit in i32.
    // Use i64 to simplify mixed arithmetic with s_ and products.
    let mut cnt = [0i64; N + 1];
    cnt[1] = 9;
    for i in 2..=N {
        cnt[i] = cnt[i - 1] * 10;
    }
    cnt
}

fn power(mut a: i64, mut k: i64) -> i64 {
    // Modular exponentiation, matching C's long long arithmetic mod MD.
    let mut p: i64 = 1;
    let mut b: i64 = a.rem_euclid(MD);
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
    // Read entire stdin, parse one integer like scanf("%d", &s).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    let cnt = init_cnt();

    // In C: ans is int; intermediate updates use % MD and can go negative.
    // Use i64 and keep the same modulo behavior (C's % keeps sign of dividend).
    let mut ans: i64 = (s - 1) / (N as i64);

    for l1 in 1..=N {
        let l1i = l1 as i64;
        if s % l1i == 0 && cnt[l1] >= s / l1i {
            ans = (ans + cnt[l1] - s / l1i + 1) % MD;
        }

        let mut s_ = s;
        for l2 in (l1 + 1)..=N {
            let l2i = l2 as i64;

            let lower = (max_i64(s_ - cnt[l2] * l2i, 1) + l1i - 1) / l1i;
            let upper = min_i64((s_ - l2i) / l1i, cnt[l1]);

            if lower <= upper {
                for r1 in 1..=l2 {
                    let r1i = r1 as i64;
                    if (s_ - r1i * l1i) % l2i == 0 {
                        let add = (upper + l2i - r1i) / l2i
                            - (lower - 1 + l2i - r1i) / l2i;
                        ans = (ans + add) % MD;
                    }
                }
            }

            if s_ / l2i < cnt[l2] {
                break;
            }
            s_ -= cnt[l2] * l2i;
        }
    }

    // for (a = 1; a <= s / a; a++)
    let mut a: i64 = 1;
    while a <= s / a {
        if s % a == 0 {
            if a > N as i64 {
                ans = (ans + power(10, a - 1) * 9 - s / a) % MD;
            }
            let other = s / a;
            if a != other && other > N as i64 {
                ans = (ans + power(10, other - 1) * 9 - a) % MD;
            }
        }
        a += 1;
    }

    if ans < 0 {
        ans += MD;
    }

    // C prints "%d\n" with ans as int; ans is in [0, MD) after adjustment.
    println!("{}", ans);
}
