fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Terminate gracefully if both n and m are zero
        if n == 0 && m == 0 {
            break;
        }

        // Initialize the relationship matrix and reachable list
        let mut relationships = vec![vec![0; 501]; 501]; // 501x501 matrix
        let mut reachable = vec![false; 501]; // List to track reachable students

        // Read m pairs of relationships
        for _ in 0..m {
            let line = reader.lines().next().unwrap().unwrap();
            let mut pair_iter = line.split_whitespace();
            let a: usize = pair_iter.next().unwrap().parse().unwrap();
            let b: usize = pair_iter.next().unwrap().parse().unwrap();
            // Store the relationship
            relationships[a][b] = 1; // Mark relationship as present
        }

        // Starting student is assumed to be 1 (as per typical problem statement)
        let start_student = 1;

        // Perform a DFS to count reachable students
        let mut count = 0;
        let mut stack = vec![start_student];

        while let Some(student) = stack.pop() {
            if !reachable[student] {
                reachable[student] = true; // Mark as visited
                for neighbor in 1..=n {
                    if relationships[student][neighbor] == 1 && !reachable[neighbor] {
                        stack.push(neighbor); // Add unvisited neighbors to stack
                    }
                }
            }
        }

        // Count reachable students excluding the starting student
        for i in 1..=n {
            if reachable[i] && i != start_student {
                count += 1;
            }
        }

        // Output the count of reachable students
        println!("{}", count);
    }
}
