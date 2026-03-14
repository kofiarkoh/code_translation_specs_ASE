use std::io::{self, BufRead};

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    // Initialize the matrix
    let mut mat = vec![vec![0; 20]; 20];
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = if i != j { 1 << 21 } else { 0 };
        }
    }

    // Read the edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut nums = edge_line.split(',');
        let a: usize = nums.next().unwrap().parse().unwrap();
        let b: usize = nums.next().unwrap().parse().unwrap();
        let c: i32 = nums.next().unwrap().parse().unwrap();
        let d: i32 = nums.next().unwrap().parse().unwrap();
        let a = a - 1; // Convert to 0-based index
        let b = b - 1; // Convert to 0-based index
        mat[a][b] = c;
        mat[b][a] = d;
    }

    // Floyd-Warshall algorithm
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                mat[j][k] = min(mat[j][k], mat[j][i] + mat[i][k]);
            }
        }
    }

    // Read the final query
    let query_line = lines.next().unwrap().unwrap();
    let mut nums = query_line.split(',');
    let a: usize = nums.next().unwrap().parse().unwrap();
    let b: usize = nums.next().unwrap().parse().unwrap();
    let c: i32 = nums.next().unwrap().parse().unwrap();
    let d: i32 = nums.next().unwrap().parse().unwrap();
    let a = a - 1; // Convert to 0-based index
    let b = b - 1; // Convert to 0-based index

    // Output the result
    println!("{}", c - mat[a][b] - mat[b][a] - d);
}
