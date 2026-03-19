use std::io::{self, Read};

const N: i32 = 8;
const MD: i64 = 1_000_000_007;

fn min_i64(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}
fn max_i64(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn power(mut a: i64, mut k: i64) -> i64 {
    let mut p: i64 = 1;
    a %= MD;
    while k != 0 {
        if (k & 1) != 0 {
            p = (p * a) % MD;
        }
        a = (a * a) % MD;
        k >>= 1;
    }
    p
}

fn main() {
    // Read single integer input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // init cnt[1..=N] where cnt[1]=9 and cnt[i]=cnt[i-1]*10
    let mut cnt = vec![0i64; (N as usize) + 1];
    cnt[1] = 9;
    for i in 2..=N as usize {
        cnt[i] = cnt[i - 1] * 10;
    }

    // The C code uses int for ans but keeps it modulo MD; we use i64 to avoid overflow.
    let mut ans: i64 = (s - 1) / (N as i64);

    for l1 in 1..=N {
        let l1i = l1 as i64;

        if s % l1i == 0 && cnt[l1 as usize] >= s / l1i {
            ans = (ans + (cnt[l1 as usize] - s / l1i + 1)) % MD;
        }

        let mut s_ = s;
        for l2 in (l1 + 1)..=N {
            let l2i = l2 as i64;

            let lower = (max_i64(s_ - cnt[l2 as usize] * l2i, 1) + l1i - 1) / l1i;
            let upper = min_i64((s_ - l2i) / l1i, cnt[l1 as usize]);

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

            if s_ / l2i < cnt[l2 as usize] {
                break;
            }
            s_ -= cnt[l2 as usize] * l2i;
        }
    }

    // Divisor scan up to sqrt(s)
    let mut a: i64 = 1;
    while a <= s / a {
        if s % a == 0 {
            if a > N as i64 {
                ans = (ans + (power(10, a - 1) * 9 - s / a)) % MD;
            }
            let other = s / a;
            if a != other && other > N as i64 {
                ans = (ans + (power(10, other - 1) * 9 - a)) % MD;
            }
        }
        a += 1;
    }

    if ans < 0 {
        ans += MD;
    }

    print!("{ans}\n");
}
