use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line containing N, M, L
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, m, l) = (parts[0], parts[1], parts[2]);

    // Edge case: if there are no locations, edges, or tasks, output 0
    if n == 0 || m == 0 || l == 0 {
        println!("0");
        return;
    }

    // Initialize distance matrix
    let mut dist = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        dist[i][i] = 0; // Distance to self is zero
    }

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let edge_parts: Vec<usize> = edge_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (u, v, d) = (edge_parts[0], edge_parts[1], edge_parts[2]);
        dist[u][v] = dist[u][v].min(d); // Store the minimum distance
        dist[v][u] = dist[v][u].min(d); // Undirected graph
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != usize::MAX && dist[k][j] != usize::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Read tasks
    let mut p = vec![0; l];
    let mut t = vec![0; l];
    for i in 0..l {
        let task_line = lines.next().unwrap().unwrap();
        let task_parts: Vec<usize> = task_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        p[i] = task_parts[0]; // Starting location for task
        t[i] = task_parts[1]; // Starting time for task
    }

    // Bipartite matching setup
    let mut match_to = vec![None; l]; // Match for each task
    let mut seen = vec![false; l]; // Seen array for DFS

    // Function to perform DFS for bipartite matching
    fn can_match(
        task: usize,
        dist: &Vec<Vec<usize>>,
        p: &Vec<usize>,
        t: &Vec<usize>,
        match_to: &mut Vec<Option<usize>>,
        seen: &mut Vec<bool>,
    ) -> bool {
        for next_task in 0..match_to.len() {
            if !seen[next_task] && dist[p[task]][p[next_task]] != usize::MAX && t[task] <= t[next_task] {
                seen[next_task] = true; // Mark this task as seen
                if match_to[next_task].is_none() || can_match(match_to[next_task].unwrap(), dist, p, t, match_to, seen) {
                    match_to[next_task] = Some(task);
                    return true;
                }
            }
        }
        false
    }

    // Count maximum matches
    let mut max_tasks = 0;
    for task in 0..l {
        seen.fill(false); // Reset seen for each task
        if can_match(task, &dist, &p, &t, &mut match_to, &mut seen) {
            max_tasks += 1; // Increment count of matched tasks
        }
    }

    // Output the result
    println!("{}", max_tasks);
}
