use std::io::{self, BufRead};

const MAX_NODES: usize = 114514;

struct Graph {
    edges: Vec<Vec<usize>>,
    incoming_count: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); n + 1],
            incoming_count: vec![0; n + 1],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
        self.incoming_count[to] += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n);
    let mut answer = vec![0; n + 1];
    let mut next_buffer = vec![Vec::new(), Vec::new()];
    let mut next_count = [0, 0];

    for _ in 0..(n - 1 + m) {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        graph.add_edge(a, b);
    }

    for i in 1..=n {
        if graph.incoming_count[i] == 0 {
            next_buffer[0].push(i);
            next_count[0] += 1;
        }
    }

    let mut phase = 0;
    while next_count[phase] > 0 {
        next_count[1 - phase] = 0;
        let current_buffer = &next_buffer[phase]; // No clone needed
        for &target in current_buffer {
            for &neighbor in &graph.edges[target] {
                if graph.incoming_count[neighbor] > 0 {
                    graph.incoming_count[neighbor] -= 1;
                    if graph.incoming_count[neighbor] == 0 {
                        answer[neighbor] = target;
                        next_buffer[1 - phase].push(neighbor);
                        next_count[1 - phase] += 1;
                    }
                }
            }
        }
        phase = 1 - phase;
        next_buffer[phase].clear(); // Clear the buffer for the next phase
    }

    for i in 1..=n {
        println!("{}", answer[i]);
    }
}
