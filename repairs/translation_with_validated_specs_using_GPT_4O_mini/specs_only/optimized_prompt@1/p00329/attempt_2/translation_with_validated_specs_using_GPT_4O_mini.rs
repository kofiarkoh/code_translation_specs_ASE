fn bipartite_matching(n: usize, grid: &[Vec<u8>]) -> (String, Vec<usize>) {
    let mut match_to = vec![None; n]; // Matches for nodes in the second set
    let mut seen = vec![false; n]; // Seen nodes during DFS

    fn dfs(node: usize, grid: &[Vec<u8>], match_to: &mut Vec<Option<usize>>, seen: &mut Vec<bool>) -> bool {
        for neighbor in 0..grid.len() {
            if grid[node][neighbor] == 1 && !seen[neighbor] {
                seen[neighbor] = true; // Mark this neighbor as seen
                if match_to[neighbor].is_none() || dfs(match_to[neighbor].unwrap(), grid, match_to, seen) {
                    match_to[neighbor] = Some(node); // Match this neighbor to the current node
                    return true;
                }
            }
        }
        false
    }

    let mut matches = 0;
    for i in 0..n {
        seen.fill(false); // Reset seen for each new node
        if dfs(i, grid, &mut match_to, &mut seen) {
            matches += 1; // Increment match count if a match is found
        }
    }

    if matches == n {
        let matched_nodes: Vec<usize> = match_to.iter()
            .map(|&m| m.unwrap() + 1) // Adjusting by +1 as per specification
            .collect();
        (String::from("yes"), matched_nodes)
    } else {
        (String::from("no"), Vec::new())
    }
}

// Example usage
fn main() {
    let n = 4; // Adjusted to match the input size
    let grid = vec![
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
    
    let (result, matches) = bipartite_matching(n, &grid);
    println!("{}", result);
    if result == "yes" {
        for m in matches {
            println!("{}", m); // Print each matched node on a new line
        }
    }
}
