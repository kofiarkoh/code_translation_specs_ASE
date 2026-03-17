fn main() {
    use std::io::{self, BufRead};

    // Read input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes in each partition
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the adjacency matrix
    let mut adj_matrix = vec![vec![0; n]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        adj_matrix[i] = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    }

    // Prepare for matching
    let mut match_pair = vec![-1; n]; // -1 indicates unmatched
    let mut seen = vec![false; n];

    // Function to perform DFS for finding augmenting paths
    fn bpm(adj_matrix: &Vec<Vec<i32>>, u: usize, match_pair: &mut Vec<i32>, seen: &mut Vec<bool>) -> bool {
        for v in 0..adj_matrix.len() {
            if adj_matrix[u][v] == 1 && !seen[v] {
                seen[v] = true; // Mark v as seen
                // If v is not matched or previously matched node has an alternate path
                if match_pair[v] == -1 || bpm(adj_matrix, match_pair[v] as usize, match_pair, seen) {
                    match_pair[v] = u as i32; // Match u with v
                    return true;
                }
            }
        }
        false
    }

    // Count of matches
    let mut result = 0;
    for u in 0..n {
        seen.fill(false); // Reset seen for each node
        if bpm(&adj_matrix, u, &mut match_pair, &mut seen) {
            result += 1; // Increment match count
        }
    }

    // Output results
    if result < n {
        println!("no");
    } else {
        let mut output = String::from("yes");
        for i in 0..n {
            output.push(' ');
            output.push_str(&(match_pair[i] + 1).to_string()); // Convert to 1-based index
        }
        println!("{}", output);
    }
}
