use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No explicit error handling specified; just exit if no input.
    };

    // Fixed internal capacity per specification.
    const CAP: usize = 100_010;
    // If n > CAP, behavior is undefined by the specification; we still allocate dynamically,
    // but do not add extra handling.
    let mut a: Vec<i64> = Vec::with_capacity(n.min(CAP));
    for _ in 0..n {
        if let Some(s) = it.next() {
            a.push(s.parse::<i64>().unwrap());
        } else {
            // No explicit error handling specified; stop early if insufficient input.
            break;
        }
    }

    // If fewer than n values were read, behavior is unspecified; proceed with what we have.
    let n = a.len();

    let mut best: i64 = 0;

    // Consider step sizes d such that 1 <= d and 2d < n-1.
    // If n < 4, no such d exists.
    if n >= 4 {
        let max_d = (n - 2) / 2; // largest d with 2d < n-1 => d <= (n-2)/2
        for d in 1..=max_d {
            let mut sum: i64 = 0;

            // Valid moves: k >= 1 while indices (k*d, n-1-k*d) are strictly inside ends
            // and do not coincide / cross.
            // This corresponds to: k*d < n-1-k*d  and k*d <= n-2 and n-1-k*d >= 1.
            // The strict inequality k*d < n-1-k*d is the binding condition.
            let mut k: usize = 1;
            loop {
                let left = k * d;
                let right = (n - 1).wrapping_sub(k * d);

                // Ensure indices are within bounds and strictly inside ends.
                if left == 0 || right == n - 1 {
                    break;
                }
                if left >= n || right >= n {
                    break;
                }
                if left >= right {
                    break;
                }

                // Accumulation is specified to be within i64 range; overflow is undefined.
                sum += a[left] + a[right];
                if sum > best {
                    best = sum;
                }

                k += 1;
            }
        }
    }

    // Output max(0, best)
    if best < 0 {
        best = 0;
    }
    print!("{best}");
}
