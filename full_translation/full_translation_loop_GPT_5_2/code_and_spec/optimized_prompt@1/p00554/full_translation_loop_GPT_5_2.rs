use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C code uses 1-based indexing and fixed arrays of size 1000.
    // We mirror that with vectors sized m+1 (index 0 unused).
    let mut a = vec![0i32; m + 1];
    let mut b = vec![0i32; m + 1];

    let mut count1: usize = 0; // number of invalidated pairs
    for i in 1..=m {
        let ai: i32 = it.next().unwrap().parse().unwrap();
        let bi: i32 = it.next().unwrap().parse().unwrap();
        a[i] = ai;
        b[i] = bi;

        // Invalidate if a[i] >= n
        if a[i] >= n {
            a[i] = 0;
            b[i] = 0;
            count1 += 1;
        }
    }

    let mut count2: i32 = 0;

    // Note: This condition matches the C code exactly: if (m - 1 > count1).
    // It is slightly unusual vs. a more direct "enough valid pairs" check,
    // but we preserve semantics as the source-of-truth behavior.
    if m.saturating_sub(1) > count1 {
        // Compute adjusted values into b: b[i] = n - a[i]
        for i in 1..=m {
            b[i] = n - a[i];
        }

        // O(m^2) sort (as in C) over indices 1..=m
        for i in 1..m {
            for j in (i + 1)..=m {
                if b[i] > b[j] {
                    b.swap(i, j);
                }
            }
        }

        // Sum the smallest (m - count1 - 1) adjusted values
        let limit = m - count1 - 1;
        for i in 1..=limit {
            count2 += b[i];
        }
    }

    print!("{count2}\n");
}
