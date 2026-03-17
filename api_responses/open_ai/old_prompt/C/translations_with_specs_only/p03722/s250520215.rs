use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes (N) and edges (M)
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the min array with "infinity" (using a large value)
    let inf = i64::MAX;
    let mut min = vec![inf; n + 1];
    min[1] = 0; // Starting node cost is 0

    // Read edges
    let mut edges = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let w: usize = parts.next().unwrap().parse().unwrap();
        let c: i64 = parts.next().unwrap().parse().unwrap();
        edges.push((u, w, -c)); // Negate cost as per specification
    }

    // Bellman-Ford algorithm
    for _ in 1..n {
        let mut updated = false;
        for &(u, w, c) in &edges {
            if min[u] != inf && min[u] + c < min[w] {
                min[w] = min[u] + c;
                updated = true;
            }
        }
        if !updated {
            break; // No updates means we can stop early
        }
    }

    // Check for negative cycles affecting the target node (node N)
    let mut has_negative_cycle = false;
    for &(u, w, c) in &edges {
        if min[u] != inf && min[u] + c < min[w] {
            has_negative_cycle = true;
            break;
        }
    }

    // Output the result
    if has_negative_cycle {
        println!("inf");
    } else {
        let result = if min[n] == inf { "inf" } else { min[n].to_string() };
        println!("{}", result);
    }
}
