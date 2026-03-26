use std::io::{self, BufRead};

fn main() {
    // Read input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the array a with size 100010
    let mut a = vec![0; 100010];

    // Read the array elements
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    // Check the condition based on n and k
    if n == k {
        // Specific condition when n equals k
        println!("Yes");
    } else {
        // Perform DFS to mark visited elements
        let mut visited = vec![false; 100010];
        let mut result = false;

        // Depth-first search function
        fn dfs(a: &mut Vec<i32>, visited: &mut Vec<bool>, index: usize) {
            if visited[index] {
                return;
            }
            visited[index] = true;
            a[index] = -1; // Mark as visited

            // Example DFS logic (this should be defined based on the actual condition)
            // Here we assume we check the next index as a simple example
            if index + 1 <= a.len() - 1 && !visited[index + 1] {
                dfs(a, visited, index + 1);
            }
        }

        // Start DFS from the first index
        dfs(&mut a, &mut visited, 1);

        // Check if any condition is met after DFS
        // This is a placeholder for the actual condition check
        for i in 1..=n {
            if a[i] == -1 {
                result = true; // Example condition
                break;
            }
        }

        // Output the result
        if result {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
