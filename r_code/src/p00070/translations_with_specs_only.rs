use std::io::{self, Read};

/// Precompute counts[k][s] = number of permutations of digits 0..9 whose first
/// k digits (k in 1..=10) produce weighted sum s = sum_{i=1..k} digit_i * i.
/// s ranges 0..=330. counts has dimension [11][331] and index 0 for k is unused
/// except we follow the specification that k==0 prints 0.
fn precompute_counts() -> Vec<Vec<u32>> {
    // counts[k][s]
    let mut counts = vec![vec![0u32; 331]; 11];

    // Start with the lexicographically smallest permutation of digits 0..9.
    let mut perm: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    loop {
        // Compute prefix weighted sums incrementally for this permutation.
        // sum_i = sum_{j=0..i} perm[j] * (j+1)
        let mut sum: usize = 0;
        for i in 0..10 {
            sum += (perm[i] as usize) * (i + 1);
            if sum <= 330 {
                counts[i + 1][sum] = counts[i + 1][sum].wrapping_add(1);
            }
            // If sum > 330 we don't record it (per spec sums considered only up to 330).
        }

        // Generate next permutation in lexicographic order.
        // If none, break.
        if !next_permutation(&mut perm) {
            break;
        }
    }

    counts
}

/// Produces the next lexicographic permutation of slice in-place.
/// Returns true if next permutation exists, false if the input was the last permutation.
///
/// This is a straightforward translation of the classic next_permutation algorithm.
fn next_permutation(a: &mut [u8]) -> bool {
    // Find longest non-increasing suffix
    if a.len() < 2 {
        return false;
    }
    let mut i = a.len() - 1;
    while i > 0 && a[i - 1] >= a[i] {
        i -= 1;
    }
    if i == 0 {
        return false; // last permutation
    }
    let pivot = i - 1;

    // Find rightmost element that exceeds the pivot
    let mut j = a.len() - 1;
    while a[j] <= a[pivot] {
        j -= 1;
    }

    a.swap(pivot, j);
    // Reverse suffix starting at i
    a[i..].reverse();
    true
}

fn main() {
    // Precompute counts before processing any queries (required by spec).
    let counts = precompute_counts();

    // Read all input from stdin as bytes, split on whitespace into tokens.
    // Per specification: input tokens are valid integers; non-integer tokens stop processing.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we fail to read stdin, there is nothing to do.
        return;
    }

    let mut outputs = String::new();

    // Parse tokens as i32. Stop at first parse error (mimics scanf behavior stopping on non-integer).
    let mut tokens = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<i32>() {
            Ok(v) => tokens.push(v),
            Err(_) => break, // stop processing further tokens on parse failure
        }
    }

    // Process pairs (n, s) until we run out of tokens. If an incomplete pair is present, ignore it.
    let mut it = tokens.chunks_exact(2);
    for pair in &mut it {
        let n = pair[0];
        let s = pair[1];

        // Following specification:
        // - If 1 <= n <= 10 and 0 <= s <= 330: print counts[n][s]
        // - If n > 10 or s > 330: print 0
        // - n == 0: print 0
        // - Negative n or s: spec states behavior is undefined; to avoid panics and remain useful,
        //   we conservatively print 0 for negative values as well. This is a narrow, safe choice
        //   that preserves the documented defined outputs.
        let out = if n == 0 {
            0u32
        } else if n < 0 || s < 0 {
            // Spec says undefined for negatives; choose to respond with 0 to avoid runtime panics.
            0u32
        } else if n > 10 || s > 330 {
            0u32
        } else {
            // Safe to index counts because we ensured 1 <= n <= 10 and 0 <= s <= 330.
            counts[n as usize][s as usize]
        };

        outputs.push_str(&format!("{}\n", out));
    }

    // Write all outputs at once.
    print!("{}", outputs);
}
