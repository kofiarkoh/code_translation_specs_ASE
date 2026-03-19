use std::io::{self, Read};

#[derive(Clone)]
struct Node {
    edge_to: Vec<usize>,   // up to 100
    edge_cost: Vec<i32>,   // non-negative
    cost: i32,             // -1 means unreachable/undiscovered
    visited: bool,
}

fn dijkstra_noheap(nodes: &mut [Node]) {
    let n = nodes.len();
    for i in 0..n {
        nodes[i].cost = -1;
        nodes[i].visited = false;
    }
    if n == 0 {
        return;
    }
    nodes[0].cost = 0;

    loop {
        let mut next_idx: Option<usize> = None;
        for i in 0..n {
            if nodes[i].visited || nodes[i].cost < 0 {
                continue;
            }
            match next_idx {
                None => next_idx = Some(i),
                Some(j) => {
                    if nodes[i].cost < nodes[j].cost {
                        next_idx = Some(i);
                    }
                }
            }
        }

        let Some(u_idx) = next_idx else { break };
        nodes[u_idx].visited = true;

        // Mirror C behavior: copy the node before relaxing edges.
        let u = nodes[u_idx].clone();
        for e in 0..u.edge_to.len() {
            let to = u.edge_to[e];
            let new_cost = u.cost + u.edge_cost[e];
            if nodes[to].cost < 0 || new_cost < nodes[to].cost {
                nodes[to].cost = new_cost;
            }
        }
    }
}

// Reads unsigned decimal integers separated by non-digit characters.
// Matches the C get_uint() behavior for well-formed inputs per spec.
struct Scanner {
    bytes: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            bytes: input.into_bytes(),
            idx: 0,
        }
    }

    fn get_uint(&mut self) -> u32 {
        // Skip non-digits
        while self.idx < self.bytes.len() && !(b'0'..=b'9').contains(&self.bytes[self.idx]) {
            self.idx += 1;
        }
        let mut n: u32 = 0;
        while self.idx < self.bytes.len() && (b'0'..=b'9').contains(&self.bytes[self.idx]) {
            n = n * 10 + (self.bytes[self.idx] - b'0') as u32;
            self.idx += 1;
        }
        n
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.get_uint() as usize;

    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node {
            edge_to: Vec::new(),
            edge_cost: Vec::new(),
            cost: -1,
            visited: false,
        })
        .collect();

    for _ in 0..n {
        let u = sc.get_uint() as usize;
        let k = sc.get_uint() as usize;

        // Spec says k in [0,100]; no explicit error handling.
        nodes[u].edge_to.clear();
        nodes[u].edge_cost.clear();
        nodes[u].edge_to.reserve(k);
        nodes[u].edge_cost.reserve(k);

        for _ in 0..k {
            let v = sc.get_uint() as usize;
            let w = sc.get_uint() as i32;
            nodes[u].edge_to.push(v);
            nodes[u].edge_cost.push(w);
        }
    }

    dijkstra_noheap(&mut nodes);

    let mut out = String::new();
    for i in 0..n {
        out.push_str(&i.to_string());
        out.push(' ');
        out.push_str(&nodes[i].cost.to_string());
        out.push('\n');
    }
    print!("{out}");
}
