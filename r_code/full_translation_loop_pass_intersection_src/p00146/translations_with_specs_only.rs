use std::f64;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Specification precondition effectively n <= 16; no explicit error handling.
    let mut a1: Vec<i64> = Vec::with_capacity(n);
    let mut a2: Vec<i64> = Vec::with_capacity(n);
    let mut a3: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        let x1: i64 = it.next().unwrap().parse().unwrap();
        let x2: i64 = it.next().unwrap().parse().unwrap();
        let x3: i64 = it.next().unwrap().parse().unwrap();
        a1.push(x1);
        a2.push(x2);
        a3.push(x3);
    }

    if n == 0 {
        // Not specified; keep behavior minimal and well-formed.
        println!();
        return;
    }

    let full_mask: usize = (1usize << n) - 1;

    // Precompute sum(third[x]) for all subsets S.
    // Use i64; preconditions state no overflow/undefined behavior.
    let mut sum_third = vec![0i64; 1usize << n];
    for mask in 1..=full_mask {
        let lsb = mask & (!mask + 1);
        let bit = lsb.trailing_zeros() as usize;
        let prev = mask ^ lsb;
        sum_third[mask] = sum_third[prev] + a3[bit];
    }

    // DP[mask][last] = minimal cost to visit exactly 'mask' and end at 'last'.
    // Use f64 as per specification; comparisons are direct.
    let inf = f64::INFINITY;
    let mut dp = vec![vec![inf; n]; 1usize << n];
    let mut pred: Vec<Vec<Option<usize>>> = vec![vec![None; n]; 1usize << n];

    // Initialize singleton sets with zero cost.
    for i in 0..n {
        let m = 1usize << i;
        dp[m][i] = 0.0;
        pred[m][i] = None;
    }

    // Expand subsets by adding one unvisited item at a time.
    for mask in 1..=full_mask {
        for last in 0..n {
            if (mask & (1usize << last)) == 0 {
                continue;
            }
            let cur_cost = dp[mask][last];
            if !cur_cost.is_finite() {
                continue;
            }

            // S includes 'last' and all previously visited items: exactly 'mask'.
            let multiplier = 70.0 + 20.0 * (sum_third[mask] as f64);

            for next in 0..n {
                if (mask & (1usize << next)) != 0 {
                    continue;
                }
                let next_mask = mask | (1usize << next);

                // Step cost = (|second[next] − second[last]| / 2000) × (70 + 20 × sum(third[x] for x in S))
                // Use floating-point division; absolute difference computed in i64 then cast.
                let dist = (a2[next] - a2[last]).abs() as f64;
                let step = (dist / 2000.0) * multiplier;

                let cand = cur_cost + step;
                if cand < dp[next_mask][next] {
                    dp[next_mask][next] = cand;
                    pred[next_mask][next] = Some(last);
                }
            }
        }
    }

    // Select minimal-cost endpoint for full_mask.
    let mut best_last = 0usize;
    let mut best_cost = dp[full_mask][0];
    for last in 1..n {
        let c = dp[full_mask][last];
        if c < best_cost {
            best_cost = c;
            best_last = last;
        }
    }

    // Reconstruct path by following predecessors while removing current endpoint from mask.
    let mut order_idx: Vec<usize> = Vec::with_capacity(n);
    let mut mask = full_mask;
    let mut last = best_last;

    loop {
        order_idx.push(last);
        let p = pred[mask][last];
        mask ^= 1usize << last;
        match p {
            Some(prev) => {
                last = prev;
            }
            None => break,
        }
    }

    order_idx.reverse();

    // Output first attributes in visit order, space-separated, newline-terminated.
    let mut out = String::new();
    for (i, &idx) in order_idx.iter().enumerate() {
        if i != 0 {
            out.push(' ');
        }
        out.push_str(&a1[idx].to_string());
    }
    out.push('\n');
    print!("{out}");
}
