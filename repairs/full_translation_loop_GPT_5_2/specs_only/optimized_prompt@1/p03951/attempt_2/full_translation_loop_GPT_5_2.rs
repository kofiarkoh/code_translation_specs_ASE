use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_i64: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    if n_i64 < 0 {
        // Undefined by specification; do nothing.
        return;
    }
    let n: usize = n_i64 as usize;

    let s = match it.next() {
        Some(x) => x.as_bytes().to_vec(),
        None => return,
    };
    let t = match it.next() {
        Some(x) => x.as_bytes().to_vec(),
        None => return,
    };

    // Preconditions say |S|=|T|=N; otherwise undefined. We assume valid input.
    // Compute the maximum overlap k such that suffix of S of length k == prefix of T of length k.
    // Then the intended result (as implied by sample) is the length of the shortest string
    // formed by appending T to S with that overlap: 2N - k.
    //
    // Do this in linear time using KMP on pattern T over text S, taking the final matched length.
    if n == 0 {
        println!("0");
        return;
    }

    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && t[i] != t[j] {
            j = pi[j - 1];
        }
        if t[i] == t[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let mut j = 0usize;
    for &c in s.iter().take(n) {
        while j > 0 && c != t[j] {
            j = pi[j - 1];
        }
        if c == t[j] {
            j += 1;
            if j == n {
                // Full match; for suffix overlap we continue with the longest proper border.
                j = pi[n - 1];
            }
        }
    }

    let overlap = j;
    let result = 2usize.saturating_mul(n).saturating_sub(overlap);
    println!("{}", result);
}
