use std::io::{self, BufRead, Write};

#[derive(Default, Clone)]
struct Node {
    edge_to: Vec<usize>,
    edge_cost: Vec<usize>,
    edge_size: usize,
    cost: isize,
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
        let next = match next {
            Some(n) => n,
            None => break,
        };
        next.visited = true;
        let u = next.clone();

        for i in 0..u.edge_size {
            let to = u.edge_to[i];
            let cost = u.cost + u.edge_cost[i] as isize;
            if nodes[to].cost < 0 || cost < nodes[to].cost {
                nodes[to].cost = cost;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut nodes = vec![Node::default(); n];

    for _ in 0..n {
        let u: usize = lines.next().unwrap().parse().unwrap();
        let k: usize = lines.next().unwrap().parse().unwrap();
        nodes[u].edge_size = k;

        for _ in 0..k {
            let to: usize = lines.next().unwrap().parse().unwrap();
            let cost: usize = lines.next().unwrap().parse().unwrap();
            nodes[u].edge_to.push(to);
            nodes[u].edge_cost.push(cost);
        }
    }

    dijkstra_noheap(&mut nodes);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..n {
        write!(handle, "{} {}\n", i, nodes[i].cost).unwrap();
    }
}
