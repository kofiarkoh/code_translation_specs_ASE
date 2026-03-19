use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn dfs(
    c: usize,
    last: Option<usize>,
    dist: i32,
    r: &[usize],          // 1-based: r[1..=R]
    d: &Vec<Vec<i32>>,    // 1-based: d[1..=N][1..=N]
    used: &mut [bool],    // 1-based: used[1..=R]
    res: &mut i32,
    r_count: usize,
) {
    if c == r_count + 1 {
        if *res > dist {
            *res = dist;
        }
        return;
    }

    for i in 1..=r_count {
        if !used[i] {
            used[i] = true;
            if let Some(las) = last {
                let add = d[r[las]][r[i]];
                dfs(c + 1, Some(i), dist + add, r, d, used, res, r_count);
            } else {
                dfs(c + 1, Some(i), 0, r, d, used, res, r_count);
            }
            used[i] = false;
        }
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let r_count: usize = it.next().unwrap().parse().unwrap();

    // Distances: 1..=N used. Initialize d[i][j] = INF for i!=j, and 0 on diagonal.
    let mut d = vec![vec![0i32; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    // Specified vertices to visit: 1-based indexing to match C code.
    let mut r = vec![0usize; r_count + 1];
    for i in 1..=r_count {
        r[i] = it.next().unwrap().parse().unwrap();
    }

    // Read edges; keep the minimum cost for parallel edges.
    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: i32 = it.next().unwrap().parse().unwrap();
        if d[a][b] > c {
            d[a][b] = c;
            d[b][a] = c;
        }
    }

    // Floyd-Warshall all-pairs shortest paths.
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                let via = d[i][k] + d[k][j];
                if d[i][j] > via {
                    d[i][j] = via;
                }
            }
        }
    }

    let mut res: i32 = INF;
    let mut used = vec![false; r_count + 1];
    dfs(1, None, 0, &r, &d, &mut used, &mut res, r_count);

    print!("{res}\n");
}
