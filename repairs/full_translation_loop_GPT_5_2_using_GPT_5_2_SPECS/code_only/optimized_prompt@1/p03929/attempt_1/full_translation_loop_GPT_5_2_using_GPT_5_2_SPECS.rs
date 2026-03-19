use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed 64-bit integers (like scanf("%lld %lld", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // In C: long long overflow is UB, but typical intent is 2's-complement wrap.
    // Use wrapping ops to emulate that behavior deterministically.
    n = n.wrapping_mul(7);

    let mut ans: i64 = 0;

    // for (i = 1; i <= n % 693; i++)
    // C's % keeps the sign of the dividend; Rust matches that for i64.
    let limit1 = n % 693;
    let mut i: i64 = 1;
    while i <= limit1 {
        if i % 7 > 1 {
            // 9 * i may overflow in C; emulate wrap.
            if (i.wrapping_mul(9) % 11) == k {
                ans = ans.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }

    let mut count: i64 = 0;
    i = 1;
    while i <= 693 {
        if i % 7 > 1 {
            if (i.wrapping_mul(9) % 11) == k {
                count = count.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }

    // ans += count * (n / 693);
    // Division in C for signed integers truncates toward zero; Rust matches.
    ans = ans.wrapping_add(count.wrapping_mul(n / 693));

    // for (i = 1; i <= 7; i++) ...
    i = 1;
    while i <= 7 {
        if i % 7 > 1 {
            if (i.wrapping_mul(9) % 11) == k {
                ans = ans.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }

    // for (i = n - 6; i <= n; i++) ...
    // Note: if n is very large, i++ could overflow in C (UB); emulate wrap.
    i = n.wrapping_sub(6);
    while i <= n {
        if i % 7 > 1 {
            if (i.wrapping_mul(9) % 11) == k {
                ans = ans.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }

    print!("{}\n", ans);
}
