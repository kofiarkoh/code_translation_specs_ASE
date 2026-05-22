use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed 64-bit integers (like scanf("%lld %lld", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // In C: long long overflow is UB; here we use wrapping to emulate typical 2's-complement behavior.
    n = n.wrapping_mul(7);

    let mut ans: i64 = 0;

    // for (i = 1; i <= n % 693; i++)
    // Note: In C, n % 693 has the sign of n. If it's negative, loop doesn't run.
    let limit1 = n % 693;
    let mut i: i64 = 1;
    while i <= limit1 {
        if i % 7 > 1 {
            // 9 * i may overflow in C (UB); use wrapping to approximate.
            let v = i.wrapping_mul(9) % 11;
            if v == k {
                ans = ans.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }

    let mut count: i64 = 0;
    i = 1;
    while i <= 693 {
        if i % 7 > 1 {
            let v = i.wrapping_mul(9) % 11;
            if v == k {
                count = count.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }

    ans = ans.wrapping_add(count.wrapping_mul(n / 693));

    i = 1;
    while i <= 7 {
        if i % 7 > 1 {
            let v = i.wrapping_mul(9) % 11;
            if v == k {
                ans = ans.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }

    // for (i = n - 6; i <= n; i++)
    // If n is very small/negative, this still follows the C loop structure.
    i = n.wrapping_sub(6);
    while i <= n {
        if i % 7 > 1 {
            let v = i.wrapping_mul(9) % 11;
            if v == k {
                ans = ans.wrapping_sub(1);
            }
        }
        i = i.wrapping_add(1);
    }

    print!("{}\n", ans);
}
