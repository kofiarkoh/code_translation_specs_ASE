use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Undefined by spec if missing; keep minimal behavior.
    };
    let k: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Undefined by spec if missing.
    };

    // Fixed internal capacity per spec; behavior undefined if exceeded.
    // We do not enforce it with explicit error handling.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Apply up to K transformations, stopping early if all elements become N.
    for _step in 0..k {
        // Difference array for range add; length n+1 to allow r+1 decrement.
        let mut diff = vec![0i64; n + 1];

        for (i0, &val) in a.iter().enumerate() {
            // Indices are 0-based here; spec is 1-based but semantics identical.
            // Influence range: [i - val, i + val] clipped to [0, n-1].
            let i = i0 as i64;
            let mut l = i - val;
            let mut r = i + val;
            if l < 0 {
                l = 0;
            }
            let n1 = n as i64 - 1;
            if r > n1 {
                r = n1;
            }

            let l_usize = l as usize;
            let r_usize = r as usize;

            diff[l_usize] += 1;
            if r_usize + 1 < diff.len() {
                diff[r_usize + 1] -= 1;
            }
        }

        let mut next = vec![0i64; n];
        let mut cur = 0i64;
        let mut all_n = true;
        let n_i64 = n as i64;

        for j in 0..n {
            cur += diff[j];
            next[j] = cur;
            if cur != n_i64 {
                all_n = false;
            }
        }

        a = next;
        if all_n {
            break;
        }
    }

    // Output final sequence.
    let mut out = String::new();
    for i in 0..n {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&a[i].to_string());
    }
    out.push('\n');
    print!("{out}");
}
