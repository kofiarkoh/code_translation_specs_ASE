use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is undefined for malformed input per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let m: usize = it.next().unwrap().parse().unwrap();
    let mut b: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        b.push(v);
    }

    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        c.push(v);
    }

    // Sort first array as required by the spec.
    a.sort();

    // Build prefix sums where prefix[i] = sum of first i elements (prefix[0] = 0).
    // Use i128 to avoid overflow; the spec does not define overflow behavior.
    let mut prefix: Vec<i128> = Vec::with_capacity(n + 1);
    prefix.push(0);
    let mut acc: i128 = 0;
    for &v in &a {
        acc += v as i128;
        prefix.push(acc);
    }

    // For each query index b[i], interpret it as an "upper bound index" into the prefix array.
    // The spec does not define indexing base or out-of-range behavior; choose the narrowest
    // reasonable interpretation:
    // - treat b[i] as a count of elements (0..=n), i.e., prefix index
    // - clamp to [0, n] to avoid panics while preserving a monotone "upper bound" meaning
    // - negative indices clamp to 0
    let mut out = String::new();
    for i in 0..m {
        let mut idx = b[i];
        if idx < 0 {
            idx = 0;
        }
        let mut idxu = idx as usize;
        if idxu > n {
            idxu = n;
        }

        let sum_at = prefix[idxu];
        let need = c[i] as i128;

        if sum_at >= need {
            out.push_str("Yes\n");
        } else {
            out.push_str("No\n");
        }
    }

    print!("{out}");
}
