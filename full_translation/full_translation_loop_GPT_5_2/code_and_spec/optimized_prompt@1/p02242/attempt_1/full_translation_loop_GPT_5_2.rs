use std::io::{self, Read};

#[derive(Clone)]
struct Node {
    edge_to: Vec<usize>,
    edge_cost: Vec<i32>,
    cost: i32,    // -1 means unreachable (as in the C code)
    visited: bool,
}

fn dijkstra_noheap(nodes: &mut [Node]) {
    let n = nodes.len();
    for i in 0..n {
        nodes[i].cost = -1;
        nodes[i].visited = false;
    }
    nodes[0].cost = 0;

    loop {
        // Find next unvisited node with smallest non-negative cost.
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

        // Snapshot u's adjacency and cost (mirrors `Node u = *next;` in C).
        let u_cost = nodes[u_idx].cost;
        let tos = nodes[u_idx].edge_to.clone();
        let costs = nodes[u_idx].edge_cost.clone();

        for (to, w) in tos.into_iter().zip(costs.into_iter()) {
            let new_cost = u_cost + w;
            let cur = nodes[to].cost;
            if cur < 0 || new_cost < cur {
                nodes[to].cost = new_cost;
            }
        }
    }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }

    fn next_i32(&mut self) -> i32 {
        // The C `get_uint` is a fast unsigned parser but returns the first
        // non-digit byte if the first char isn't a digit. Per spec, inputs are
        // valid non-negative integers, so we implement a standard skip+parse.
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_digit() {
            n = n * 10 + (self.buf[self.idx] - b'0') as i32;
            self.idx += 1;
        }
        n
    }

    fn next_usize(&mut self) -> usize {
        self.next_i32() as usize
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_usize();

    // In C, `Node node[n];` leaves fields uninitialized, but the program only
    // relies on `edge_size` being set for each `u` read. The input format
    // provides each node's adjacency, so we initialize safely here.
    let mut nodes = vec![
        Node {
            edge_to: Vec::new(),
            edge_cost: Vec::new(),
            cost: -1,
            visited: false,
        };
        n
    ];

    for _ in 0..n {
        let u = sc.next_usize();
        let k = sc.next_usize();
        nodes[u].edge_to.clear();
        nodes[u].edge_cost.clear();
        nodes[u].edge_to.reserve(k);
        nodes[u].edge_cost.reserve(k);
        for _ in 0..k {
            let v = sc.next_usize();
            let c = sc.next_i32();
            nodes[u].edge_to.push(v);
            nodes[u].edge_cost.push(c);
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
