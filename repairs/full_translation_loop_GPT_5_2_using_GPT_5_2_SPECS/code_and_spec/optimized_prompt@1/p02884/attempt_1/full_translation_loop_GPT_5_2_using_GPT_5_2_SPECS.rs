use std::io::{self, Read};

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut m: usize = it.next().unwrap().parse().unwrap();

    // C uses fixed-size arrays; we allocate based on n.
    // Adjacency matrix c[i][j] == true iff edge i->j exists.
    let mut c = vec![vec![false; n + 1]; n + 1];

    while m > 0 {
        let i: usize = it.next().unwrap().parse().unwrap();
        let j: usize = it.next().unwrap().parse().unwrap();
        c[i][j] = true;
        m -= 1;
    }

    // e[i] stores the selected outgoing neighbor to remove for node i (or -1 if none).
    // Selection rule (from C): if outdegree>1, pick neighbor b with maximal d[b]
    // (ties keep the first encountered due to strict < comparison).
    let mut e: Vec<isize> = vec![-1; n + 1];

    // Compute expected steps d[i] for original graph.
    let mut d = vec![1e9f64; n + 1];
    d[n] = 0.0;

    for i in (1..=n - 1).rev() {
        let mut s = 0.0f64;
        let mut a_cnt: usize = 0;
        let mut b: usize = 0;

        for j in i..=n {
            if c[i][j] {
                s += d[j] + 1.0;
                if b == 0 || d[b] < d[j] {
                    b = j;
                }
                a_cnt += 1;
            }
        }

        // Per spec, if a_cnt==0 behavior is undefined (division by zero in C).
        d[i] = s / (a_cnt as f64);
        if a_cnt > 1 {
            e[i] = b as isize;
        }
    }

    let mut min_val = d[1];

    // Evaluate scenarios: remove exactly one selected edge from a node with >=2 outgoing edges.
    for k in 1..=n {
        if e[k] == -1 {
            continue;
        }
        let removed_to = e[k] as usize;

        // Recompute expectations with that single edge removed.
        let mut d2 = vec![1e9f64; n + 1];
        d2[n] = 0.0;

        for i in (1..=n - 1).rev() {
            let mut s = 0.0f64;
            let mut a_cnt: usize = 0;

            for j in i..=n {
                if !c[i][j] {
                    continue;
                }
                if i == k && j == removed_to {
                    continue;
                }
                s += d2[j] + 1.0;
                a_cnt += 1;
            }

            // Per spec, if a_cnt==0 behavior is undefined.
            d2[i] = s / (a_cnt as f64);
        }

        if d2[1] < min_val {
            min_val = d2[1];
        }
    }

    // Match C formatting exactly: "%.10lf\n"
    println!("{:.10}", min_val);
}
