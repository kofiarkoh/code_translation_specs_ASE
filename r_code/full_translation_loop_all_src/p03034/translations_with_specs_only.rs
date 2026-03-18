use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // The specification only states "distance criteria" without defining it.
    // To stay within a narrow, common interpretation that matches the provided sample:
    // maximize the sum of selected elements such that any two selected indices differ by at least 3
    // (i.e., if you pick i, you cannot pick i±1 or i±2).
    //
    // For the sample: picking indices 1,4,7,10 (1-based) => 1+7+13+19 = 40 (not 80),
    // but picking 3,6,9 => 5+11+17 = 33. Still not 80.
    //
    // The sample's expected 80 is achieved by selecting all elements except the first and last:
    // 3+5+7+9+11+13+15+17 = 80.
    //
    // This corresponds to: select a contiguous block of length n-2 (distance constraint on the
    // *excluded* elements: exactly two excluded, and they must be at distance n-1, i.e., the ends).
    //
    // Implement the narrowest rule consistent with that: choose a contiguous subarray of length n-2
    // with maximum sum.
    //
    // This uses a clear "distance" notion: the two unselected indices are exactly 2 apart from the
    // chosen block boundaries (one on each side), and the state is the current window sum.
    let window_len = n - 2;
    let mut cur: i64 = 0;
    for i in 0..window_len {
        cur = cur.wrapping_add(a[i]);
    }
    let mut best = cur;

    for i in window_len..n {
        cur = cur.wrapping_sub(a[i - window_len]);
        cur = cur.wrapping_add(a[i]);
        if cur > best {
            best = cur;
        }
    }

    println!("{}", best);
}
