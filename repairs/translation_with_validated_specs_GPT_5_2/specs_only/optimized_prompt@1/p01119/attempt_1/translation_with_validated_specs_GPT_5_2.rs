use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }
        let m: usize = it.next().unwrap().parse().unwrap();

        let mut targets: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            targets.push(it.next().unwrap().parse::<i32>().unwrap());
        }

        let mut weights: Vec<i32> = Vec::with_capacity(m);
        for _ in 0..m {
            weights.push(it.next().unwrap().parse::<i32>().unwrap());
        }

        // Enumerate all signed net outcomes Q by assigning each weight a coefficient in {-1,0,+1}.
        // Practical constraints: m <= 12, so 3^m <= 531441.
        let mut q_vals: Vec<i32> = Vec::new();
        q_vals.push(0);
        for &w in &weights {
            let prev = q_vals.clone();
            q_vals.reserve(prev.len() * 2);
            for &s in &prev {
                // Intermediate arithmetic is assumed to fit in i32; if it doesn't, behavior is unspecified.
                q_vals.push(s + w);
                q_vals.push(s - w);
            }
        }

        // Determine which targets are "exactly balanceable" per spec:
        // exists q == t AND t > 0. Note: t == 0 is never treated as exactly balanceable.
        let mut unbalanceable: Vec<i32> = Vec::new();
        for &t in &targets {
            let mut ok = false;
            if t > 0 {
                for &q in &q_vals {
                    if q == t {
                        ok = true;
                        break;
                    }
                }
            }
            if !ok {
                unbalanceable.push(t);
            }
        }

        if unbalanceable.is_empty() {
            println!("0");
            continue;
        }

        // For each unbalanceable target t, candidate deviations are { |t - q| : q in Q }.
        // We need the minimum d in the intersection across all such targets.
        //
        // Narrow, faithful approach: compute per-target deviation sets and intersect.
        // Use sorting + dedup to represent sets without assuming any external bounds.
        let mut intersection: Option<Vec<i32>> = None;

        for &t in &unbalanceable {
            let mut ds: Vec<i32> = Vec::with_capacity(q_vals.len());
            for &q in &q_vals {
                // Use i64 for subtraction/abs to avoid accidental i32 overflow in Rust;
                // spec says behavior is unspecified on overflow, but inputs are assumed to fit i32.
                let diff = (t as i64) - (q as i64);
                let ad = diff.abs() as i32;
                ds.push(ad);
            }
            ds.sort_unstable();
            ds.dedup();

            intersection = Some(match intersection {
                None => ds,
                Some(prev) => {
                    // Intersect two sorted unique vectors.
                    let mut i = 0usize;
                    let mut j = 0usize;
                    let mut out: Vec<i32> = Vec::new();
                    while i < prev.len() && j < ds.len() {
                        let a = prev[i];
                        let b = ds[j];
                        if a == b {
                            out.push(a);
                            i += 1;
                            j += 1;
                        } else if a < b {
                            i += 1;
                        } else {
                            j += 1;
                        }
                    }
                    out
                }
            });

            if intersection.as_ref().unwrap().is_empty() {
                break;
            }
        }

        match intersection {
            Some(v) if !v.is_empty() => {
                // Minimum non-negative integer d in the intersection.
                println!("{}", v[0]);
            }
            _ => {
                println!("-1");
            }
        }
    }
}
