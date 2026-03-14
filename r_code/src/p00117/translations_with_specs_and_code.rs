use std::io::{self, BufRead};

const INF: i32 = 1 << 21; // Represents a large value (infinity)
const MAX_NODES: usize = 20;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of nodes and edges
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the weight matrix
    let mut mat = [[INF; MAX_NODES]; MAX_NODES];
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = if i == j { 0 } else { INF };
        }
    }

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split(',');
        let a: usize = edge_parts.next().unwrap().parse().unwrap();
        let b: usize = edge_parts.next().unwrap().parse().unwrap();
        let c: i32 = edge_parts.next().unwrap().parse().unwrap();
        let d: i32 = edge_parts.next().unwrap().parse().unwrap();
        
        // Adjust for 0-based indexing
        let a = a - 1;
        let b = b - 1;

        mat[a][b] = c;
        mat[b][a] = d;
    }

    // Floyd-Warshall algorithm to compute shortest paths
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                mat[j][i] = min(mat[j][i], mat[j][k] + mat[k][i]);
            }
        }
    }

    // Read the final cost query
    let cost_line = lines.next().unwrap().unwrap();
    let mut cost_parts = cost_line.split(',');
    let a: usize = cost_parts.next().unwrap().parse().unwrap();
    let b: usize = cost_parts.next().unwrap().parse().unwrap();
    let c: i32 = cost_parts.next().unwrap().parse().unwrap();
    let d: i32 = cost_parts.next().unwrap().parse().unwrap();

    // Adjust for 0-based indexing
    let a = a - 1;
    let b = b - 1;

    // Calculate the adjusted cost
    let adjusted_cost = c - mat[a][b] - mat[b][a] - d;
    println!("{}", adjusted_cost);
}
