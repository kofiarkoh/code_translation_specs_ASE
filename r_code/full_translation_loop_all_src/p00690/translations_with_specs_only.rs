use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 {
            // Terminating dataset: m is read but not otherwise used.
            break;
        }

        let mut edges: Vec<Edge> = Vec::with_capacity(m);
        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let w: i64 = it.next().unwrap().parse().unwrap();
            edges.push(Edge { u: a, v: b, w });
        }

        // Fixed internal capacities per specification assumptions.
        // If exceeded, behavior is unspecified; we do not add extra handling.
        let mut used = vec![false; edges.len()];

        let mut best_len: i64 = 0;
        let mut best_path: Vec<usize> = Vec::new();

        // Depth-first exploration of all edge-simple trails.
        fn dfs(
            cur: usize,
            cur_len: i64,
            path: &mut Vec<usize>,
            edges: &[Edge],
            used: &mut [bool],
            best_len: &mut i64,
            best_path: &mut Vec<usize>,
        ) {
            if cur_len > *best_len {
                *best_len = cur_len;
                best_path.clear();
                best_path.extend_from_slice(path);
            }

            for (i, e) in edges.iter().enumerate() {
                if used[i] {
                    continue;
                }
                let next = if e.u == cur {
                    e.v
                } else if e.v == cur {
                    e.u
                } else {
                    continue;
                };

                used[i] = true;
                path.push(next);
                dfs(next, cur_len + e.w, path, edges, used, best_len, best_path);
                path.pop();
                used[i] = false;
            }
        }

        // Start from each available edge in both directions.
        for (i, e) in edges.iter().enumerate() {
            if used[i] {
                continue;
            }

            // Direction u -> v
            used[i] = true;
            let mut path = Vec::new();
            path.push(e.u);
            path.push(e.v);
            dfs(e.v, e.w, &mut path, &edges, &mut used, &mut best_len, &mut best_path);
            used[i] = false;

            // Direction v -> u
            used[i] = true;
            let mut path2 = Vec::new();
            path2.push(e.v);
            path2.push(e.u);
            dfs(e.u, e.w, &mut path2, &edges, &mut used, &mut best_len, &mut best_path);
            used[i] = false;
        }

        // Output: exactly two lines per dataset.
        // Edge case: if no route with at least one edge is found, best_len remains 0.
        // The vertex sequence line is not meaningfully defined; we still attempt to print
        // a starting vertex label. We choose "1" as the narrowest reasonable default.
        println!("{}", best_len);
        if best_path.is_empty() {
            println!("1");
        } else {
            for (idx, v) in best_path.iter().enumerate() {
                if idx > 0 {
                    print!(" ");
                }
                print!("{}", v);
            }
            println!();
        }
    }
}
