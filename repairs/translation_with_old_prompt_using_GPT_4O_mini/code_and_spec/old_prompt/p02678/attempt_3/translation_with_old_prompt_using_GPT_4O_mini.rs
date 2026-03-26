use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Edge {
    to: usize,
    next: Option<Box<Edge>>,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N and M
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Initialize the graph and predecessor array
    let mut graph: Vec<Option<Box<Edge>>> = vec![None; n + 1];
    let mut predecessors = vec![-1; n + 1];

    // Read edges and construct the graph
    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let a: usize = edge_parts.next().unwrap().parse().unwrap();
        let b: usize = edge_parts.next().unwrap().parse().unwrap();
        push(&mut graph, a, b);
        push(&mut graph, b, a);
    }

    // BFS initialization
    predecessors[1] = 0; // Starting node
    let mut queue = VecDeque::new();
    queue.push_back(1);

    // BFS traversal
    while let Some(i) = queue.pop_front() {
        let mut current_edge = graph[i].as_ref();
        while let Some(edge) = current_edge {
            let j = edge.to;
            if predecessors[j] == -1 {
                predecessors[j] = i as isize; // Store predecessor
                queue.push_back(j);
            }
            current_edge = edge.next.as_ref();
        }
    }

    // Output results
    println!("Yes");
    for i in 2..=n {
        println!("{}", predecessors[i]);
    }
}

// Function to add an edge to the graph
fn push(graph: &mut Vec<Option<Box<Edge>>>, from: usize, to: usize) {
    let new_edge = Box::new(Edge {
        to,
        next: graph[from].take(),
    });
    graph[from] = Some(new_edge);
}
