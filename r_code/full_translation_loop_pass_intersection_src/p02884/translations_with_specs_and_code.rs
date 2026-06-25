use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // C uses fixed-size arrays; we allocate based on n (1..=n used).
    // c[u][v] is 1 if edge u->v exists.
    let mut c = vec![vec![0u8; n + 1]; n + 1];

    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        c[u][v] = 1;
    }

    // d[i] = expected steps from i to terminal n under current graph.
    // e[i] = candidate edge destination for node i, or -1 if none.
    let mut d = vec![1e9f64; n + 1];
    let mut e = vec![-1isize; n + 1];

    // Original graph evaluation (matches C: only considers edges i->j for j in i..=n).
    d[n] = 0.0;
    for i in (1..=n - 1).rev() {
        let mut s = 0.0f64;
        let mut a = 0usize;
        let mut b: usize = 0; // destination with largest d[j] among available edges
        for j in i..=n {
            if c[i][j] != 0 {
                s += d[j] + 1.0;
                if b == 0 || d[b] < d[j] {
                    b = j;
                }
                a += 1;
            }
        }
        // Per spec, behavior is undefined if a==0; we mirror C (would divide by zero).
        d[i] = s / (a as f64);
        if a > 1 {
            e[i] = b as isize;
        }
    }

    let mut best = d[1];

    // Try removing each candidate edge (k -> e[k]) one at a time and recompute.
    for k in 1..=n {
        if e[k] == -1 {
            continue;
        }
        let ek = e[k] as usize;

        // Reset d
        for x in 0..n {
            d[x] = 1e9;
        }
        d[n] = 0.0;

        for i in (1..=n - 1).rev() {
            let mut s = 0.0f64;
            let mut a = 0usize;
            for j in i..=n {
                if c[i][j] == 0 {
                    continue;
                }
                if i == k && j == ek {
                    continue; // remove this one edge
                }
                s += d[j] + 1.0;
                a += 1;
            }
            d[i] = s / (a as f64);
        }

        if d[1] < best {
            best = d[1];
        }
    }

    // Fixed precision output exactly as C.
    println!("{:.10}", best);
}
