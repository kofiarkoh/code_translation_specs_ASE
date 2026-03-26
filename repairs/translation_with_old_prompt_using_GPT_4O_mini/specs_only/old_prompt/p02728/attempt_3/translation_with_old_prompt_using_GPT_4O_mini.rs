use std::io::{self, BufRead};

fn main() {
    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of nodes
    let v: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read parent-child relationships for edges a and b
    let mut a = Vec::new();
    let mut b = Vec::new();
    
    for _ in 0..(v - 1) {
        let edge: Vec<usize> = lines.next().unwrap().unwrap()
            .trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        a.push(edge[0]);
        b.push(edge[1]);
    }

    // Initialize DP arrays
    let mut tans = vec![0i64; v]; // Results for each node
    let mut tS = vec![0i64; v];   // Temporary sums
    let mut tu = vec![0; v];      // Child count

    // Build adjacency list from edges
    let mut tree = vec![Vec::new(); v];
    for i in 0..(v - 1) {
        tree[a[i]].push(b[i]);
        tree[b[i]].push(a[i]);
    }

    // Perform DFS to calculate DP values
    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, tans: &mut Vec<i64>, tS: &mut Vec<i64>, tu: &mut Vec<usize>) {
        for &child in &tree[node] {
            if child != parent {
                dfs(child, node, tree, tans, tS, tu);
                tu[node] += 1; // Count children
                tS[node] += tans[child]; // Sum contributions from children
            }
        }
        tans[node] = tS[node] + 1; // Example calculation, adjust as needed
    }

    // Start DFS from the root node (assuming node 0 is the root)
    dfs(0, usize::MAX, &tree, &mut tans, &mut tS, &mut tu);

    // Output results
    for result in tans {
        println!("{}", result);
    }
}
