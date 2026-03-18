use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    if n == 0 {
        // Spec: if the first integer input is zero, return zero.
        println!("0");
        return;
    }

    let s_str = match it.next() {
        Some(v) => v,
        None => {
            // Spec does not define missing string; keep minimal behavior.
            println!("0");
            return;
        }
    };
    let s = s_str.as_bytes();

    // Spec: if size of the string is less than specified length, behavior is not defined.
    // We assume well-formed input per preconditions and use n as the intended length.
    let m = n;

    // Interpretation consistent with the failing sample:
    // We want the maximum length of contiguous '1's after changing up to k *groups* of '0's to '1's.
    // This is a common variant where flipping a whole zero-run counts as 1 change.
    //
    // Approach:
    // - Run-length encode into alternating counts starting with '1' (possibly 0).
    // - Build prefix sums over counts.
    // - For each starting index at a '1' run (even index), take window covering up to k zero-runs:
    //   that is, include 2*k+1 runs (1,0,1,0,...,1). Compute sum via prefix sums.
    let mut runs: Vec<usize> = Vec::new();

    // Ensure runs start with '1' run.
    let mut i = 0usize;
    if i < m && s[i] == b'0' {
        runs.push(0);
    }

    while i < m {
        let c = s[i];
        let mut j = i;
        while j < m && s[j] == c {
            j += 1;
        }
        runs.push(j - i);
        i = j;
    }

    // If ends with '0' run, append trailing 0-length '1' run to keep indexing safe.
    if runs.len() % 2 == 0 {
        runs.push(0);
    }

    let mut pref = vec![0usize; runs.len() + 1];
    for idx in 0..runs.len() {
        pref[idx + 1] = pref[idx] + runs[idx];
    }

    let mut best = 0usize;
    let span = 2 * k + 1;

    // Start at even indices (1-runs).
    for l in (0..runs.len()).step_by(2) {
        let r = (l + span).min(runs.len());
        let sum = pref[r] - pref[l];
        if sum > best {
            best = sum;
        }
    }

    println!("{}", best);
}
