use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // No explicit error handling per specification; unwrap() is acceptable under preconditions.
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Fixed-size internal storage constraint (capacity limit).
    // The specification does not provide an exact limit; choose a narrow, reasonable fixed bound.
    // Behavior is unspecified if lengths exceed internal capacity.
    const CAP: usize = 10000;
    if n > CAP || m > CAP {
        // Unspecified behavior; we choose to proceed with allocation anyway would violate "fixed-size".
        // Exiting early keeps behavior minimal without inventing recovery.
        return;
    }

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut b: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        b.push(it.next().unwrap().parse().unwrap());
    }

    // Compute maximum over all starting indices in b of the longest prefix of b[start..]
    // that can be matched as a subsequence of a with strictly increasing positions.
    let mut best: usize = 0;

    for start in 0..m {
        let mut i: usize = 0; // current scan position in a
        let mut matched: usize = 0;

        for j in start..m {
            // Find next occurrence of b[j] in a at position >= i.
            while i < n && a[i] != b[j] {
                i += 1;
            }
            if i == n {
                break; // cannot match further
            }
            // Match at position i; next match must be strictly after.
            matched += 1;
            i += 1;
        }

        if matched > best {
            best = matched; // invariant: best never decreases
        }
    }

    // Output is in [0, m].
    println!("{}", best);
}
