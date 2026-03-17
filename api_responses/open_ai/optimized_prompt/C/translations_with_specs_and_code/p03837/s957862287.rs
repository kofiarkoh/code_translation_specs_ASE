use std::io::{self, BufRead};

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read number of vertices (N) and edges (M)
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut a = vec![0; m];
    let mut b = vec![0; m];
    let mut c = vec![0; m];
    let mut dist = vec![vec![i32::MAX; n]; n];

    // Read edges
    for i in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        a[i] = parts.next().unwrap().parse::<usize>().unwrap() - 1; // 0-based index
        b[i] = parts.next().unwrap().parse::<usize>().unwrap() - 1; // 0-based index
        c[i] = parts.next().unwrap().parse().unwrap();
    }

    // Initialize distance matrix
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Set initial distances based on edges
    for i in 0..m {
        dist[a[i]][b[i]] = min(dist[a[i]][b[i]], c[i]);
        dist[b[i]][a[i]] = min(dist[b[i]][a[i]], c[i]);
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = m;

    // Count edges not part of any shortest path
    for i in 0..m {
        let mut is_part_of_shortest_path = false;
        for j in 0..n {
            if dist[j][a[i]] + c[i] == dist[j][b[i]] {
                is_part_of_shortest_path = true;
                break;
            }
        }
        if !is_part_of_shortest_path {
            ans -= 1;
        }
    }

    // Output the result
    println!("{}", ans);
}
