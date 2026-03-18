use std::io::{self, Read};

fn main() {
    // Read all input (undefined behavior on malformed input per spec; we unwrap).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a = vec![0i32; n];
    let mut b = vec![0i32; n];
    let mut c = vec![0i32; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
        b[i] = it.next().unwrap().parse().unwrap();
        c[i] = it.next().unwrap().parse().unwrap();
    }

    // Handle n == 0: C code would misbehave; spec allows non-negative n.
    // We output nothing (no newline) to avoid inventing extra output.
    if n == 0 {
        return;
    }

    // The original C code uses fixed arrays sized for n <= 16 (d[20][1<<16]).
    // We preserve the same intended DP over subsets; for n > 16, the C code
    // would go out of bounds (undefined). We do not add explicit error handling.
    let size = 1usize << n;

    let inf = 1e18f64;
    let mut d = vec![vec![inf; size]; n];
    let mut fr = vec![vec![-1i32; size]; n];

    // Cost function f(i, j, k) as in C.
    // Note: In the C code, if j == 0 it returns 0, but in the DP j is never 0
    // for reachable states. We keep the same behavior.
    let cost = |i: usize, mask: usize, k: usize, b: &Vec<i32>, c: &Vec<i32>, n: usize| -> f64 {
        if mask == 0 {
            return 0.0;
        }
        let mut w: i32 = 0;
        for l in 0..n {
            if (mask & (1usize << l)) != 0 {
                w += c[l] * 20;
            }
        }
        let diff = (b[k] - b[i]).abs() as f64;
        (diff / 2000.0) * ((w + 70) as f64)
    };

    for i in 0..n {
        d[i][1usize << i] = 0.0;
    }

    for mask in 0..size {
        for i in 0..n {
            // C bug: `if((1<<i)&j==0)` is parsed as `(1<<i) & (j==0)`,
            // so it fails to properly skip states. The intended semantics are
            // clearly "if i not in mask, continue", which matches the spec's DP.
            if (mask & (1usize << i)) == 0 {
                continue;
            }
            let base = d[i][mask];
            if base >= inf / 2.0 {
                continue;
            }
            for k in 0..n {
                if (mask & (1usize << k)) != 0 {
                    continue;
                }
                let next_mask = mask | (1usize << k);
                let cand = base + cost(i, mask, k, &b, &c, n);
                if d[k][next_mask] > cand {
                    d[k][next_mask] = cand;
                    fr[k][next_mask] = i as i32;
                }
            }
        }
    }

    let full = size - 1;

    // Find best ending node.
    let mut best_j: usize = 0;
    for i in 0..n {
        if d[i][full] < d[best_j][full] {
            best_j = i;
        }
    }

    // Reconstruct path.
    let mut r = vec![0usize; n];
    r[n - 1] = best_j;
    let mut mask = full;
    for t in 1..n {
        let cur = r[n - t];
        let prev = fr[cur][mask];
        // In well-formed DP, prev must exist for t < n.
        r[n - t - 1] = prev as usize;
        mask &= !(1usize << cur);
    }

    // Output a[r[i]] with spaces, newline at end, matching C formatting.
    let mut out = String::new();
    for i in 0..n {
        if i + 1 == n {
            out.push_str(&format!("{}\n", a[r[i]]));
        } else {
            out.push_str(&format!("{} ", a[r[i]]));
        }
    }
    print!("{out}");
}
