use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: usize = it.next().unwrap().parse().unwrap();

    // C code uses a counting array c[0..=N] but only initializes/sorts indices 1..=N.
    // It then increments c[A[i]]; behavior is undefined if A[i] is out of bounds.
    // We preserve that assumption (spec: "valid range that can be indexed").
    let mut c = vec![0i64; n + 1];
    for _ in 0..n {
        let a: usize = it.next().unwrap().parse().unwrap();
        c[a] += 1;
    }

    // Sort counts in non-decreasing order for indices 1..=N (i.e., c[1..=N]).
    // This matches mergeSort(c, 1, N+1) in the C code.
    c[1..=n].sort_unstable();

    // Sum counts of the first (N-K) entries in the sorted slice, but only if > 0.
    // Note: The C code has an unused variable `t`; we omit it.
    let limit = n.saturating_sub(k);
    let mut ans: i64 = 0;
    for i in 1..=limit {
        if c[i] > 0 {
            ans += c[i];
        }
    }

    print!("{ans}\n");
}
