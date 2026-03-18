use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k_in: i64 = it.next().unwrap().parse().unwrap();

    let mut p: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        // Specification says P values are valid indices; common convention is 1-based in such tasks.
        // We interpret them as 1-based and convert to 0-based.
        let v: i64 = it.next().unwrap().parse().unwrap();
        p.push((v - 1) as usize);
    }

    let mut c: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        c.push(v);
    }

    // If K == 0, no operations can be performed; return a specific minimum value.
    // The specification does not define the exact minimum; we use i64::MIN as the narrowest sentinel.
    if k_in <= 0 {
        println!("{}", i64::MIN);
        return;
    }

    let k = k_in as i64;

    // Compute maximum value over all starting positions, following permutation P for up to K steps.
    // This matches the typical "positions and costs" cycle-walk semantics with O(N^2) behavior.
    let mut ans: i64 = i64::MIN;

    for start in 0..n {
        // Build the cycle path starting from `start` until it repeats.
        let mut path: Vec<i64> = Vec::new();
        let mut cur = start;
        loop {
            cur = p[cur];
            path.push(c[cur]);
            if cur == start {
                break;
            }
            // If P is not a permutation, this could loop forever; preconditions imply valid indices,
            // but not necessarily permutation. We still guard by length N to keep bounded behavior.
            if path.len() > n {
                break;
            }
        }

        if path.is_empty() {
            continue;
        }

        let m = path.len() as i64;
        let cycle_sum: i64 = path.iter().sum();

        // For each possible number of steps taken within one cycle prefix (1..=m),
        // consider taking additional full cycles if beneficial and within K.
        let mut prefix_sum: i64 = 0;
        for t in 1..=m {
            prefix_sum += path[(t - 1) as usize];

            if t > k {
                break;
            }

            // Base: take exactly t steps.
            ans = ans.max(prefix_sum);

            // If cycle sum is positive, we can add full cycles.
            if cycle_sum > 0 {
                let remaining = k - t;
                let full = remaining / m;
                let candidate = prefix_sum + full * cycle_sum;
                ans = ans.max(candidate);
            }
        }
    }

    println!("{}", ans);
}
