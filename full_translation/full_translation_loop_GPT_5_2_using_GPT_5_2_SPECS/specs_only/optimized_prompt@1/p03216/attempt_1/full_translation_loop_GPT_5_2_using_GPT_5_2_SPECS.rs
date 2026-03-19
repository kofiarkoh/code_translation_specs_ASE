use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let s = it.next().unwrap_or("");
    // Preconditions say s length matches n; if violated, behavior is undefined.
    let bytes = s.as_bytes();

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut queries: Vec<usize> = Vec::with_capacity(q);
    for _ in 0..q {
        let k: usize = it.next().unwrap().parse().unwrap();
        queries.push(k);
    }

    let mut out = String::new();

    for &k in &queries {
        // Preconditions guarantee 0 <= k <= n; otherwise undefined.
        if k == 0 || k > n {
            // Narrowest reasonable behavior: no windows => sum is 0.
            // (If k>n, spec says undefined; we still avoid panics.)
            out.push_str("0\n");
            continue;
        }

        // For each window [l, r) of length k, count D-M-C triples with a<b<c.
        // Use O(n*k) per query as allowed by spec's non-functional constraints.
        let mut total: i64 = 0;

        for l in 0..=n - k {
            let r = l + k;

            // Count triples in this window:
            // For each M position, add (#D before it) * (#C after it).
            let mut d_before: i64 = 0;

            // Precompute suffix counts of 'C' within the window.
            // suffix_c[i] = number of 'C' in bytes[i..r] (i in [l..=r])
            let mut suffix_c: Vec<i64> = vec![0; (k + 1)];
            // Map window index j in [0..k] to global index l+j.
            for j_rev in (0..k).rev() {
                let idx = l + j_rev;
                let add = if bytes[idx] == b'C' { 1 } else { 0 };
                suffix_c[j_rev] = suffix_c[j_rev + 1] + add;
            }

            let mut window_triples: i64 = 0;
            for j in 0..k {
                let idx = l + j;
                match bytes[idx] {
                    b'D' => {
                        d_before += 1;
                    }
                    b'M' => {
                        let c_after = suffix_c[j + 1];
                        window_triples = window_triples.wrapping_add(d_before.wrapping_mul(c_after));
                    }
                    _ => {}
                }
            }

            total = total.wrapping_add(window_triples);
        }

        out.push_str(&format!("{total}\n"));
    }

    print!("{out}");
}
