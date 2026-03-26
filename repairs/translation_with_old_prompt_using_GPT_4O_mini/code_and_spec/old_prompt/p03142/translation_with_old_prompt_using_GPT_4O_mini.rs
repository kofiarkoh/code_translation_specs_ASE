use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX_NODES: usize = 114514;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse()?;
    let m: usize = parts.next().unwrap().parse()?;

    if n == 0 || m == 0 {
        return Err("Invalid input".into()); // Invalid input
    }

    let mut ec = vec![0; n + 1];
    let mut es: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut sansyo = vec![0; n + 1];
    let mut answer = vec![0; n + 1];

    for _ in 0..(n - 1 + m) {
        let edge_line = lines.next().unwrap()?;
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().unwrap().parse()?;
        let b: usize = edge_parts.next().unwrap().parse()?;

        if a < 1 || a > n || b < 1 || b > n {
            return Err("Invalid input".into()); // Invalid input
        }

        // Add edge
        es[a].push(b);
        ec[a] += 1;
        sansyo[b] += 1;
    }

    let mut next_buffer: [VecDeque<usize>; 2] = [VecDeque::new(), VecDeque::new()];
    let mut next_count = [0, 0];

    for i in 1..=n {
        if sansyo[i] == 0 {
            next_buffer[0].push_back(i);
            next_count[0] += 1;
        }
    }

    let mut phase = 0;
    while next_count[phase] > 0 {
        next_count[1 - phase] = 0;
        while let Some(target) = next_buffer[phase].pop_front() {
            for &neighbor in &es[target] {
                sansyo[neighbor] -= 1;
                if sansyo[neighbor] == 0 {
                    answer[neighbor] = target;
                    next_buffer[1 - phase].push_back(neighbor);
                    next_count[1 - phase] += 1;
                }
            }
        }
        phase = 1 - phase;
    }

    for i in 1..=n {
        println!("{}", answer[i]);
    }

    Ok(()) // Successful execution
}
