use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut v: Vec<(i64, i64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let a: i64 = it.next().unwrap().parse().unwrap(); // long long
        let b: i64 = it.next().unwrap().parse().unwrap(); // int (kept as i64 for sums)
        v.push((a, b));
    }

    // Invariant: sorted by the first element.
    v.sort_by_key(|&(a, _)| a);

    // Maintain cumulative sum of second elements.
    // Track minimum of (a_i - prefix_sum_before_i) and maximum computed value.
    //
    // Let S_k be prefix sum of b for first k elements (S_0 = 0).
    // For each element i (1-based), consider:
    //   value_i = a_i - min_{j < i} (a_j - S_{j-1}) + S_{i-1}
    // This uses sorted order, updates cumulative sums, and tracks min/max.
    //
    // This is the narrowest interpretation consistent with the spec that also matches
    // the provided sample.
    let mut prefix_before: i128 = 0; // S_{i-1}
    let mut min_key: i128 = i128::MAX;
    let mut best: i128 = i128::MIN;

    for (a, b) in v {
        let a128 = a as i128;

        if min_key != i128::MAX {
            let candidate = a128 - min_key + prefix_before;
            if candidate > best {
                best = candidate;
            }
        }

        // Update min_key using current element as potential "minimum" for future elements.
        // key_i = a_i - S_{i-1}
        let key = a128 - prefix_before;
        if key < min_key {
            min_key = key;
        }

        // Update cumulative sum of second elements.
        prefix_before += b as i128;
    }

    // If N == 1, no pair (j < i) exists; best remains MIN. In that case, output 0
    // as the narrowest deterministic choice (spec does not define this corner).
    if best == i128::MIN {
        println!("0");
        return;
    }

    // Output as long long (i64). Clamp deterministically if out of range.
    let out: i64 = if best > i64::MAX as i128 {
        i64::MAX
    } else if best < i64::MIN as i128 {
        i64::MIN
    } else {
        best as i64
    };

    println!("{}", out);
}
