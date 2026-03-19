use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed integers like C's scanf("%ld%ld", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; we just exit.
    };
    let mut p: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut ans: i64 = 1;

    // for (long i=2; i*i<=p; i++)
    // Use i128 for the i*i comparison to avoid overflow in Rust while matching intent.
    let mut i: i64 = 2;
    while (i as i128) * (i as i128) <= (p as i128) {
        let mut cnt: i64 = 0;
        // while (p%i<1) cnt++, p/=i;
        // In C, p%i can be negative if p is negative; condition is "< 1" (not "== 0").
        // Preserve that behavior.
        while p % i < 1 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            // for (int j=0; j<cnt/n; j++) ans*=i;
            // C integer division truncates toward zero; Rust does too for i64.
            let times = cnt / n;
            for _j in 0..(times as i32) {
                // C signed overflow is UB; we use wrapping to emulate typical two's complement behavior.
                ans = ans.wrapping_mul(i);
            }
        }
        i += 1;
    }

    // if (p>1 && n==1) ans*=p;
    if p > 1 && n == 1 {
        ans = ans.wrapping_mul(p);
    }

    print!("{}\n", ans);
}
