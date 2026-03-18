use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers (C code uses long; assume i64).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut p: i64 = it.next().unwrap().parse().unwrap();

    // C initializes ans=1 and never outputs 0; this conflicts with SPEC invariant #6
    // ("output will be zero if no valid prime factors meet the specified conditions").
    // We follow the C code's actual behavior as the only concrete source of truth here.
    let mut ans: i64 = 1;

    let mut i: i64 = 2;
    while i * i <= p {
        let mut cnt: i64 = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n && n != 0 {
            // Multiply by i exactly floor(cnt / n) times.
            let times = cnt / n;
            for _ in 0..times {
                ans *= i;
            }
        }
        i += 1;
    }

    // Remaining prime factor (if any) is only multiplied in when n == 1, per C code.
    if p > 1 && n == 1 {
        ans *= p;
    }

    print!("{ans}\n");
}
