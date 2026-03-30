use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX_NODES: usize = 114514;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N and M
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid input for N");
    let m: usize = parts.next().unwrap().parse().expect("Invalid input for M");

    // Initialize structures
    let mut es: Vec<Vec<usize>> = vec![Vec::new(); MAX_NODES];
    let mut sansyo = vec![0; MAX_NODES];
    let mut next_buffer = [VecDeque::new(), VecDeque::new()];
    let mut answer = vec![0; MAX_NODES];

    // Read edges
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().unwrap().parse().expect("Invalid input for A");
        let b: usize = edge_parts.next().unwrap().parse().expect("Invalid input for B");

        // Add edge
        es[a].push(b);
        sansyo[b] += 1;
    }

    // Initialize the first buffer with nodes having no incoming edges
    for i in 1..=n {
        if sansyo[i] == 0 {
            next_buffer[0].push_back(i);
        }
    }

    let mut phase = 0;
    let mut index = 1; // To track the order of nodes in the answer
    while !next_buffer[phase].is_empty() {
        next_buffer[1 - phase].clear(); // Clear the next buffer for the next phase

        while let Some(target) = next_buffer[phase].pop_front() {
            answer[target] = index; // Assign the current index to the node
            index += 1; // Increment the index for the next node

            for &next_node in &es[target] {
                sansyo[next_node] -= 1;
                if sansyo[next_node] == 0 {
                    next_buffer[1 - phase].push_back(next_node);
                }
            }
        }
        phase = 1 - phase; // Switch phases
    }

    // Output the answer
    for i in 1..=n {
        println!("{}", answer[i]);
    }
}
