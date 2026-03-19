use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Preconditions in spec: effectively n <= 16 (bitmask DP).
    // We do not add explicit error handling; behavior is undefined if violated.
    let mut a = vec![0i32; n];
    let mut b = vec![0i32; n];
    let mut c = vec![0i32; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
        b[i] = it.next().unwrap().parse().unwrap();
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let states = 1usize << n;
    let inf = 1e18f64;

    // d[last][mask] = best cost to visit exactly 'mask' and end at 'last'
    let mut d = vec![vec![inf; states]; n];
    let mut fr = vec![vec![-1i32; states]; n];

    for i in 0..n {
        d[i][1usize << i] = 0.0;
    }

    // Cost function per spec/C:
    // move i -> k given visited-set mask j (before adding k):
    // abs(b[k]-b[i])/2000.0 * (70 + 20*sum_{l in j} c[l])
    // Note: C has `if (j==0) return 0;` but j is never 0 for reachable states.
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
        let dist = (b[k] - b[i]).abs() as f64;
        dist / 2000.0 * ((w + 70) as f64)
    };

    // DP over subsets
    for mask in 0..states {
        for i in 0..n {
            // IMPORTANT: The C code has `if((1<<i)&j==0)continue;`
            // which is a precedence bug in C (parsed as (1<<i) & (j==0)).
            // The SPEC states intended semantics: "visited iff included in mask".
            // Therefore we implement the intended check:
            if (mask & (1usize << i)) == 0 {
                continue;
            }
            let di = d[i][mask];
            if di >= inf / 2.0 {
                continue;
            }
            for k in 0..n {
                if (mask & (1usize << k)) != 0 {
                    continue;
                }
                let nmask = mask | (1usize << k);
                let cand = di + cost(i, mask, k, &b, &c, n);
                if cand < d[k][nmask] {
                    d[k][nmask] = cand;
                    fr[k][nmask] = i as i32;
                }
            }
        }
    }

    // Choose best ending node for full mask
    let full = states - 1;
    let mut best_last = 0usize;
    for i in 0..n {
        if d[i][full] < d[best_last][full] {
            best_last = i;
        }
    }

    // Reconstruct path (order of indices)
    let mut r = vec![0usize; n];
    r[n - 1] = best_last;
    let mut mask = full;
    for t in 1..n {
        let cur = r[n - t];
        let prev = fr[cur][mask];
        // Under preconditions, prev must exist for t < n.
        r[n - t - 1] = prev as usize;
        mask &= !(1usize << cur);
    }

    // Output identifiers in order, space-separated, newline-terminated
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
