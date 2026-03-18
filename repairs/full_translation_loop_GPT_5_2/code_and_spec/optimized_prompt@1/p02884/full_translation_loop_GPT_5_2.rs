use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let mut m: i32 = it.next().unwrap().parse().unwrap();

    // C code uses fixed-size arrays; we size dynamically.
    // c[i][j] is treated as boolean adjacency (0/1).
    let mut c = vec![vec![false; n + 1]; n + 1];

    while m > 0 {
        m -= 1;
        let i: usize = it.next().unwrap().parse().unwrap();
        let j: usize = it.next().unwrap().parse().unwrap();
        if i <= n && j <= n {
            c[i][j] = true;
        }
        // Spec says edges are valid; we still guard to avoid panic on bad input.
    }

    let inf = 1e9_f64;

    // d[0..=n], with d[n] = 0, others initialized to inf.
    let mut d = vec![inf; n + 1];
    d[n] = 0.0;

    // e[0..=n], initialized to -1; stores the chosen edge endpoint to remove.
    let mut e = vec![-1isize; n + 1];

    // First pass: compute baseline d[i] and candidate edge e[i]
    // Note: This code assumes that for each i, there is at least one outgoing edge
    // to some j in [i..=n], otherwise it would divide by zero (as in the C code).
    for i in (1..=n.saturating_sub(1)).rev() {
        let mut s = 0.0_f64;
        let mut a_cnt: i32 = 0;
        let mut b: usize = 0; // 0 means "unset" (matches C's b==0 check)

        for j in i..=n {
            if c[i][j] {
                s += d[j] + 1.0;
                if b == 0 || d[b] < d[j] {
                    b = j;
                }
                a_cnt += 1;
            }
        }

        d[i] = s / (a_cnt as f64);
        if a_cnt > 1 {
            e[i] = b as isize;
        }
    }

    let mut min_val = if n >= 1 { d[1] } else { inf };

    // Try removing one specific edge (k -> e[k]) for each k where e[k] is set.
    for k in 1..=n {
        if e[k] == -1 {
            continue;
        }
        let removed_to = e[k] as usize;

        // Recompute d with that edge removed
        for x in 0..=n {
            d[x] = inf;
        }
        d[n] = 0.0;

        for i in (1..=n.saturating_sub(1)).rev() {
            let mut s = 0.0_f64;
            let mut a_cnt: i32 = 0;

            for j in i..=n {
                // Skip if no edge, or if it's the removed edge.
                if !c[i][j] || (i == k && j == removed_to) {
                    continue;
                }
                s += d[j] + 1.0;
                a_cnt += 1;
            }

            d[i] = s / (a_cnt as f64);
        }

        if n >= 1 && d[1] < min_val {
            min_val = d[1];
        }
    }

    // Output with exactly ten decimal places, matching C's printf("%.10lf\n", ...)
    println!("{:.10}", min_val);
}
