use std::cmp::max;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    w: usize,
    s: usize,
    v: i64,
}

fn main() {
    // Read entire stdin (no explicit error handling per spec/C behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // malformed/incomplete input: behavior not defined; exit quietly
    };

    let mut items: Vec<Item> = Vec::with_capacity(n);
    for _ in 0..n {
        let w: usize = it.next().unwrap().parse().unwrap();
        let s: usize = it.next().unwrap().parse().unwrap();
        let v: i64 = it.next().unwrap().parse().unwrap();
        items.push(Item { w, s, v });
    }

    // Sort by (s + w) ascending, matching C comparator semantics.
    // Note: In C, (p->s+p->w)-(q->s+q->w) is an int subtraction; here we use a safe ordering.
    items.sort_by_key(|x| x.s + x.w);

    const M: usize = 20000;
    let mut dp = vec![0i64; M + 1];

    for item in items {
        // C loops j from item.s down to 0 inclusive.
        // Preconditions imply item.s + item.w <= 20000, so k is in-bounds.
        for j in (0..=item.s).rev() {
            let k = j + item.w;
            dp[k] = max(dp[k], dp[j] + item.v);
        }
    }

    let mut ans: i64 = 0;
    for &x in &dp {
        ans = max(ans, x);
    }

    print!("{ans}\n");
}
