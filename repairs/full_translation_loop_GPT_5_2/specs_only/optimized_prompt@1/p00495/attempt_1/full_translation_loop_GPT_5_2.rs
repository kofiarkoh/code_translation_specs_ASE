use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Specification: two integers representing lengths (non-negative; if negative => undefined).
    // We parse as i64 first to avoid accidental wrap; if negative, behavior is undefined by spec,
    // so we do not attempt to handle it beyond letting parsing/logic proceed naturally.
    let n1: i64 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return, // No input; unspecified. Exit quietly.
    };
    let n2: i64 = it.next().unwrap().parse().unwrap();

    // Narrowest reasonable interpretation: lengths are counts of elements to read.
    // If fewer elements exist than specified, behavior is undefined by spec; unwrap() will panic.
    let n1u: usize = n1 as usize;
    let n2u: usize = n2 as usize;

    let mut a = Vec::with_capacity(n1u);
    for _ in 0..n1u {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let mut b = Vec::with_capacity(n2u);
    for _ in 0..n2u {
        let v: i64 = it.next().unwrap().parse().unwrap();
        b.push(v);
    }

    // Functional interpretation consistent with the spec:
    // For each starting position s in the second sequence b, compute the length of the longest
    // subsequence of b[s..] that can be formed using elements from a, where "using elements"
    // means matching in order against a (i.e., b-subsequence must be a subsequence of a).
    // Then output the maximum length over all s.
    //
    // This is equivalent to: max over s of LCS_length(a, b[s..]) when the LCS is constrained
    // to be a subsequence of b[s..] and a (standard LCS). But since we only need subsequence
    // of b[s..] that appears in a, the maximum length is the length of the longest common
    // subsequence between a and b[s..].
    //
    // We implement LCS length with O(|a|*|b[s..]|) DP using two rows to respect space limits.

    let mut best: usize = 0;

    for s in 0..=n2u {
        // b_suffix is b[s..]
        let m = n2u - s;
        if m == 0 {
            // Empty suffix => LCS length 0; no update needed.
            continue;
        }

        // Early bound: cannot exceed min(n1u, m)
        if best >= n1u.min(m) {
            continue;
        }

        let mut prev = vec![0usize; m + 1];
        let mut curr = vec![0usize; m + 1];

        for i in 0..n1u {
            curr[0] = 0;
            for j in 0..m {
                if a[i] == b[s + j] {
                    curr[j + 1] = prev[j] + 1;
                } else {
                    let x = prev[j + 1];
                    let y = curr[j];
                    curr[j + 1] = if x >= y { x } else { y };
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        let l = prev[m];
        if l > best {
            best = l;
        }
    }

    // Output: an integer representing the maximum length found (non-negative).
    println!("{}", best);
}
