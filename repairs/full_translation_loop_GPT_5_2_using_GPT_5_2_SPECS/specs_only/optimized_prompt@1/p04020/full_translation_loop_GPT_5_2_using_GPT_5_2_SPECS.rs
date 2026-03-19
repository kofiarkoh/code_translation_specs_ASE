use std::io::{self, Read};

fn main() {
    // Read all input; behavior is undefined for malformed/missing input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    // Capacity constraint is a precondition; we do not enforce it (undefined otherwise).
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut ans: i64 = 0;

    // Process from first to last, allowing at most one cross-position pair per boundary.
    for i in 0..n {
        // Form as many within-position pairs as possible.
        // For negative values, Rust's / and % follow truncation toward zero; spec says meaning
        // is not guaranteed then, so we keep direct arithmetic behavior.
        ans += a[i] / 2;
        a[i] %= 2;

        // Optionally form one cross-position pair with successor:
        // requires current leftover == 1 and successor strictly > 1 at that moment.
        if i + 1 < n && a[i] == 1 && a[i + 1] > 1 {
            ans += 1;
            a[i] = 0;
            a[i + 1] -= 1; // reduce successor by exactly 1 only when it is strictly > 1
        }
    }

    print!("{ans}");
}
