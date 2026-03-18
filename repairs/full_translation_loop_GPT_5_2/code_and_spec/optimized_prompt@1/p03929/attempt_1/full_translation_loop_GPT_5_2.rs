use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut n: i128 = it.next().unwrap().parse::<i128>().unwrap();
    let k: i128 = it.next().unwrap().parse::<i128>().unwrap();

    // C code uses long long and does: n *= 7;
    // Spec says n should be non-negative; we follow that intended domain.
    n *= 7;

    let mut ans: i128 = 0;

    // Helper closure: condition used repeatedly
    let valid = |i: i128, k: i128| -> bool {
        (i % 7) > 1 && ((9 * i) % 11) == k
    };

    // First partial block: i = 1..= n % 693
    let r = n % 693;
    for i in 1..=r {
        if valid(i, k) {
            ans += 1;
        }
    }

    // Count within one full period 1..=693
    let mut count: i128 = 0;
    for i in 1..=693 {
        if valid(i, k) {
            count += 1;
        }
    }

    ans += count * (n / 693);

    // Subtract for i = 1..=7
    for i in 1..=7 {
        if valid(i, k) {
            ans -= 1;
        }
    }

    // Subtract for i = n-6 ..= n (inclusive)
    // Note: In C, if n < 6, the loop starts negative and still runs up to n.
    // We preserve that behavior using i128.
    for i in (n - 6)..=n {
        if valid(i, k) {
            ans -= 1;
        }
    }

    // Output as integer with newline, matching C printf("%lld\n", ans);
    // Spec guarantees it fits in long long; cast accordingly.
    println!("{}", ans as i64);
}
