use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => return,
    };
    let k: usize = match it.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => {
            println!("0");
            return;
        }
    };

    // Read N integers; if fewer are provided, treat missing as absent (narrowest behavior).
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(t) = it.next() {
            a.push(t.parse().unwrap_or(0));
        } else {
            break;
        }
    }

    // Preconditions / early termination per spec.
    if n < k {
        println!("0");
        return;
    }

    // If K == 0, "contains all integers from 1 to K" is vacuously true.
    // The smallest contiguous subarray length is 0 (empty window).
    if k == 0 {
        println!("0");
        return;
    }

    // Sliding window: maintain counts for values 1..=K.
    let mut counts = vec![0usize; k + 1]; // index 0 unused
    let mut have = 0usize; // number of distinct required values currently present (count > 0)

    let mut best: Option<usize> = None;
    let mut left: usize = 0;

    for right in 0..a.len() {
        let v = a[right];
        if v >= 1 && (v as usize) <= k {
            let idx = v as usize;
            if counts[idx] == 0 {
                have += 1;
            }
            counts[idx] += 1;
        }

        // When window contains all required integers, try to shrink from the left.
        while have == k && left <= right {
            let len = right - left + 1;
            best = Some(best.map_or(len, |b| b.min(len)));

            let lv = a[left];
            if lv >= 1 && (lv as usize) <= k {
                let idx = lv as usize;
                counts[idx] -= 1;
                if counts[idx] == 0 {
                    have -= 1;
                }
            }
            left += 1;
        }
    }

    // If no valid subarray exists, output 0.
    println!("{}", best.unwrap_or(0));
}
