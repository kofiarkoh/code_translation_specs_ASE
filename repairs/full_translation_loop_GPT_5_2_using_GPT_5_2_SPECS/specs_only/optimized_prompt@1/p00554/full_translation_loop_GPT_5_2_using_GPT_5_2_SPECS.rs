use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say input is present; no explicit error handling required.
    let reference: i64 = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity: at most 999 pairs.
    // Preconditions guarantee n <= 999, so we don't need to guard.
    let mut deficits: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        let first: i64 = it.next().unwrap().parse().unwrap();
        let _second: i64 = it.next().unwrap().parse().unwrap();

        if first < reference {
            // Preconditions guarantee this subtraction does not overflow.
            deficits.push(reference - first);
        }
    }

    let k = deficits.len();
    if k <= 1 {
        println!("0");
        return;
    }

    // Need sum of smallest k-1 values in S.
    // Quadratic ordering behavior is acceptable per spec; use a simple selection sort
    // for the first k-1 elements to avoid assuming more than necessary.
    for i in 0..(k - 1) {
        let mut min_idx = i;
        for j in (i + 1)..k {
            if deficits[j] < deficits[min_idx] {
                min_idx = j;
            }
        }
        deficits.swap(i, min_idx);
    }

    let mut sum: i64 = 0;
    for i in 0..(k - 1) {
        sum += deficits[i];
    }

    // Output is non-negative by construction (deficits are positive).
    println!("{}", sum);
}
