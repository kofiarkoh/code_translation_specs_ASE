use std::collections::VecDeque;
use std::io::{self, Write};

struct Edge {
    to: usize,
    next: Option<Box<Edge>>,
}

struct Graph {
    edges: Vec<Option<Box<Edge>>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![None; n + 1], // 1-indexed
        }
    }

    fn push(&mut self, from: usize, to: usize) {
        let new_edge = Box::new(Edge {
            to,
            next: self.edges[from].take(),
        });
        self.edges[from] = Some(new_edge);
    }
}

fn bfs(graph: &Graph, n: usize) -> Vec<i32> {
    let mut parent = vec![-1; n + 1]; // -1 indicates unvisited
    let mut queue = VecDeque::new();
    
    parent[1] = 0; // Starting node has no parent
    queue.push_back(1);

    while let Some(current) = queue.pop_front() {
        if let Some(mut edge) = graph.edges[current].clone() {
            while let Some(next_edge) = edge {
                let j = next_edge.to;
                if parent[j] == -1 { // If unvisited
                    parent[j] = current as i32; // Set parent
                    queue.push_back(j);
                }
                edge = next_edge.next; // Move to the next edge
            }
        }
    }
    parent
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input for M");

    let mut graph = Graph::new(n);

    for _ in 0..m {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let mut edge_iter = input.split_whitespace();
        let a: usize = edge_iter.next().unwrap().parse().expect("Invalid input for edge");
        let b: usize = edge_iter.next().unwrap().parse().expect("Invalid input for edge");
        graph.push(a, b);
        graph.push(b, a);
    }

    let parent = bfs(&graph, n);
    
    println!("Yes");
    for i in 2..=n {
        if parent[i] != -1 {
            println!("{}", parent[i]);
        }
    }
}
```

### Notes:
1. **Graph Representation**: The graph is represented using a vector of `Option<Box<Edge>>`, which allows for a linked list-like structure for edges while maintaining safety.
2. **BFS Implementation**: The BFS function uses a `VecDeque` for efficient queue operations, and it tracks parent nodes in a vector initialized to -1.
3. **Input Handling**: The program reads input using `stdin` and handles parsing with error checks to ensure valid integers.
4. **Output**: The output format matches the specification, including the confirmation message and the parent nodes for reachable nodes.
5. **Safety**: The code avoids `unsafe` blocks, ensuring memory safety and preventing potential undefined behavior.
