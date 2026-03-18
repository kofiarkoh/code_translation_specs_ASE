use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        // Specification: no explicit handling for allocation failures.
        Self {
            adj: vec![Vec::new(); vertices],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize) {
        // Specification: invalid indices may lead to undefined behavior.
        // In Rust, indexing out of bounds panics; we keep direct indexing semantics.
        self.adj[src].push(dst);
    }

    fn has_cycle(&self) -> bool {
        // Standard directed cycle detection via DFS with 3-color marking:
        // 0 = unvisited, 1 = visiting (in recursion stack), 2 = done.
        let n = self.adj.len();
        let mut state = vec![0u8; n];

        fn dfs(u: usize, g: &Graph, state: &mut [u8]) -> bool {
            state[u] = 1;
            for &v in &g.adj[u] {
                match state[v] {
                    0 => {
                        if dfs(v, g, state) {
                            return true;
                        }
                    }
                    1 => return true, // back-edge => cycle
                    _ => {}            // 2 => already processed
                }
            }
            state[u] = 2;
            false
        }

        for u in 0..n {
            if state[u] == 0 && dfs(u, self, &mut state) {
                return true;
            }
        }
        false
    }
}

fn main() {
    // Input format per specification:
    // vertices edges (then edges pairs: src dst) repeated edges times.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let vertices: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // no input => no observable output required by spec
    };
    let edges: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut g = Graph::new(vertices);

    for _ in 0..edges {
        let src: usize = it.next().unwrap().parse().unwrap();
        let dst: usize = it.next().unwrap().parse().unwrap();
        g.add_edge(src, dst);
    }

    let out = if g.has_cycle() { 1 } else { 0 };
    print!("{out}");
}
