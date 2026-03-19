use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Collect all lines from the input at once to avoid moving `reader`
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut line_iter = lines.into_iter();

    while let Some(line) = line_iter.next() {
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Terminate gracefully if both n and m are zero
        if n == 0 && m == 0 {
            break;
        }

        // Initialize the relationship matrix and reachable students
        let mut relationships = vec![vec![false; 501]; 501];
        let mut reachable = vec![false; 501];

        // Read the relationships
        for _ in 0..m {
            let line = line_iter.next().unwrap();
            let mut pair = line.split_whitespace();
            let a: usize = pair.next().unwrap().parse().unwrap();
            let b: usize = pair.next().unwrap().parse().unwrap();
            relationships[a][b] = true; // Mark the relationship
            relationships[b][a] = true; // Assuming undirected relationships
        }

        // Read the starting student (assumed to be the first student, index 1)
        let start_student = 1;
        reachable[start_student] = true; // Mark the starting student as reachable

        // Perform a simple DFS to count reachable students
        let mut stack = vec![start_student];
        while let Some(student) = stack.pop() {
            for neighbor in 1..=n {
                if relationships[student][neighbor] && !reachable[neighbor] {
                    reachable[neighbor] = true; // Mark as reachable
                    stack.push(neighbor); // Add to stack for further exploration
                }
            }
        }

        // Count reachable students excluding the starting student
        let count = reachable.iter().skip(1).take(n - 1).filter(|&&r| r).count();
        println!("{}", count);
    }
}
