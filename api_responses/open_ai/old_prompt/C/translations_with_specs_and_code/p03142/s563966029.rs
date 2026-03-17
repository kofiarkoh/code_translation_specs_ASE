use std::io::{self, BufRead};

const MAX_NODES: usize = 114514;

struct Graph {
    edges: Vec<Vec<usize>>,
    in_degree: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![Vec::new(); n + 1],
            in_degree: vec![0; n + 1],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges[from].push(to);
        self.in_degree[to] += 1;
    }
}

fn topological_sort(graph: &Graph, n: usize) -> Vec<usize> {
    let mut next_buffer = vec![Vec::new(), Vec::new()];
    let mut next_count = [0, 0];
    let mut answer = vec![0; n + 1];

    for i in 1..=n {
        if graph.in_degree[i] == 0 {
            next_buffer[0].push(i);
            next_count[0] += 1;
        }
    }

    let mut phase = 0;
    while next_count[phase] > 0 {
        next_count[1 - phase] = 0;
        for &target in &next_buffer[phase] {
            for &neighbor in &graph.edges[target] {
                if graph.in_degree[neighbor] > 0 {
                    graph.in_degree[neighbor] -= 1;
                    if graph.in_degree[neighbor] == 0 {
                        answer[neighbor] = target;
                        next_buffer[1 - phase].push(neighbor);
                        next_count[1 - phase] += 1;
                    }
                }
            }
        }
        phase = 1 - phase;
    }

    answer
}

fn main() -> Result<(), i32> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().ok_or(1)??;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().ok_or(1)?.parse().map_err(|_| 1)?;
    let m: usize = parts.next().ok_or(1)?.parse().map_err(|_| 1)?;

    let mut graph = Graph::new(n);

    for _ in 0..(n - 1 + m) {
        let line = lines.next().ok_or(1)??;
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().ok_or(1)?.parse().map_err(|_| 1)?;
        let b: usize = parts.next().ok_or(1)?.parse().map_err(|_| 1)?;
        graph.add_edge(a, b);
    }

    let answer = topological_sort(&graph, n);
    for i in 1..=n {
        println!("{}", answer[i]);
    }

    Ok(())
}
