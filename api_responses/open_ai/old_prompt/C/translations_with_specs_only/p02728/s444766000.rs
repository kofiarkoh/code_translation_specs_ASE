use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of nodes
    let mut lines = reader.lines();
    let v: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Initialize parent-child relationships
    let mut a = vec![0; v - 1];
    let mut b = vec![0; v - 1];
    
    // Read the first set of edges
    for i in 0..v - 1 {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Read the second set of edges
    for i in 0..v - 1 {
        b[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Initialize results and state variables
    let mut tans = vec![0i64; v]; // Results for each node
    let mut tS = vec![0i64; v];   // State variable for contributions
    let mut tu = vec![0i64; v];   // Another state variable for contributions

    // Build adjacency list for the tree
    let mut tree = vec![Vec::new(); v];
    for i in 0..v - 1 {
        tree[a[i] as usize].push(b[i] as usize);
        tree[b[i] as usize].push(a[i] as usize);
    }

    // Perform tree DP calculations
    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, tans: &mut Vec<i64>, tS: &mut Vec<i64>, tu: &mut Vec<i64>) {
        tS[node] = 1; // Initialize contribution count for the node
        for &child in &tree[node] {
            if child != parent {
                dfs(child, node, tree, tans, tS, tu);
                tS[node] += tS[child]; // Aggregate contributions from children
                tans[node] += tans[child]; // Aggregate results from children
            }
        }
        tans[node] += tS[node]; // Example calculation for the node
    }

    // Start DFS from the root node (assumed to be node 0)
    dfs(0, usize::MAX, &tree, &mut tans, &mut tS, &mut tu);

    // Output results for each node
    for i in 0..v {
        println!("{}", tans[i]);
    }
}
