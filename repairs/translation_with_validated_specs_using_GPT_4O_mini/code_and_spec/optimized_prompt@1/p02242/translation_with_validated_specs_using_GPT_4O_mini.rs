use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
struct Node {
    edge_to: Vec<usize>,
    edge_cost: Vec<i32>,
    edge_size: usize,
    cost: i32,
    visited: bool,
}

fn dijkstra_noheap(nodes: &mut [Node]) {
    let n = nodes.len();
    for node in nodes.iter_mut() {
        node.cost = -1;
        node.visited = false;
    }
    nodes[0].cost = 0;

    loop {
        let mut next: Option<&mut Node> = None;
        for node in nodes.iter_mut() {
            if node.visited || node.cost < 0 {
                continue;
            }
            if next.is_none() || node.cost < next.as_ref().unwrap().cost {
                next = Some(node);
            }
        }
        if next.is_none() {
            break;
        }
        let next_node = next.unwrap();
        next_node.visited = true;
        let u = next_node.clone(); // Clone the node to avoid borrowing issues
        for i in 0..u.edge_size {
            let to = u.edge_to[i];
            let cost = u.cost + u.edge_cost[i];
            if nodes[to].cost < 0 || cost < nodes[to].cost {
                nodes[to].cost = cost;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut nodes: Vec<Node> = vec![Node { edge_to: Vec::new(), edge_cost: Vec::new(), edge_size: 0, cost: -1, visited: false }; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        nodes[u].edge_size = k;
        for _ in 0..k {
            let to: usize = parts.next().unwrap().parse().unwrap();
            let cost: i32 = parts.next().unwrap().parse().unwrap();
            nodes[u].edge_to.push(to);
            nodes[u].edge_cost.push(cost);
        }
    }

    dijkstra_noheap(&mut nodes);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, node) in nodes.iter().enumerate() {
        write!(handle, "{} {}\n", i, node.cost).unwrap();
    }
}
