use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let n: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break, // No more datasets.
        };
        let m: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break, // Incomplete dataset; unspecified behavior, so just stop.
        };

        if n == 0 && m == 0 {
            break;
        }

        // Relationship matrix: fixed-size 501x501 as per constraints.
        // Use u8 to represent 0/1 adjacency; indices are 1..=n.
        let mut rel = vec![vec![0u8; 501]; 501];

        for _ in 0..m {
            let a: usize = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            // Specification does not state directed vs undirected explicitly.
            // Narrowest reasonable interpretation for "relationships between students"
            // and "connected through direct or indirect relationships" is undirected.
            rel[a][b] = 1;
            rel[b][a] = 1;
        }

        // Starting student is "designated" but not explicitly given in input.
        // Narrowest reasonable interpretation consistent with typical indexing is student 1.
        let start: usize = 1;

        // Track reachable students; reset per dataset.
        let mut visited = vec![false; 501];
        let mut q = VecDeque::new();

        visited[start] = true;
        q.push_back(start);

        while let Some(u) = q.pop_front() {
            for v in 1..=n {
                if rel[u][v] != 0 && !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }

        // Count reachable excluding the starting student itself.
        let mut count = 0usize;
        for v in 1..=n {
            if v != start && visited[v] {
                count += 1;
            }
        }

        println!("{count}");
    }
}
