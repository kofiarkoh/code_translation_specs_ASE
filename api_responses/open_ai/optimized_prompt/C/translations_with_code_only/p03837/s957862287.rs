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
    let mut lines = stdin.lock().lines();

    // Read N and M
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut a = vec![0; m as usize];
    let mut b = vec![0; m as usize];
    let mut c = vec![0; m as usize];
    let mut dist = vec![vec![1_000_000_000; n as usize]; n as usize];

    // Read edges
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edge = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a[i as usize] = edge.next().unwrap() - 1; // 0-based index
        b[i as usize] = edge.next().unwrap() - 1; // 0-based index
        c[i as usize] = edge.next().unwrap();
    }

    // Initialize distance matrix
    for i in 0..n {
        dist[i as usize][i as usize] = 0;
    }

    // Fill in the distances based on edges
    for i in 0..m {
        dist[a[i as usize] as usize][b[i as usize] as usize] = min(dist[a[i as usize] as usize][b[i as usize] as usize], c[i as usize]);
        dist[b[i as usize] as usize][a[i as usize] as usize] = min(dist[b[i as usize] as usize][a[i as usize] as usize], c[i as usize]);
    }

    // Floyd-Warshall algorithm
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i as usize][j as usize] = min(dist[i as usize][j as usize], dist[i as usize][k as usize] + dist[k as usize][j as usize]);
            }
        }
    }

    let mut ans = m;

    // Check for redundant edges
    for i in 0..m {
        let mut f = 1;
        for j in 0..n {
            if dist[j as usize][a[i as usize] as usize] + c[i as usize] == dist[j as usize][b[i as usize] as usize] {
                f = 0;
            }
        }
        if f == 0 {
            ans -= 1;
        }
    }

    // Output the result
    println!("{}", ans);
}
