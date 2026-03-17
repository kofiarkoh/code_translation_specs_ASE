use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
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
        let mut next: Option<usize> = None;
        for (i, node) in nodes.iter().enumerate() {
            if node.visited || node.cost < 0 {
                continue;
            }
            if next.is_none() || nodes[next.unwrap()].cost > node.cost {
                next = Some(i);
            }
        }
        let next_index = match next {
            Some(index) => index,
            None => break,
        };
        nodes[next_index].visited = true;
        let u = &nodes[next_index];

        for i in 0..u.edge_size {
            let to = u.edge_to[i];
            let cost = u.cost + u.edge_cost[i] as isize;
            if nodes[to].cost < 0 || cost < nodes[to].cost {
                nodes[to].cost = cost;
            }
        }
    }
}

fn get_uint() -> usize {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn put_uint(n: usize) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", n).unwrap();
}

fn main() {
    let n = get_uint();
    let mut nodes: Vec<Node> = vec![Node {
        edge_to: Vec::new(),
        edge_cost: Vec::new(),
        edge_size: 0,
        cost: -1,
        visited: false,
    }; n];

    for _ in 0..n {
        let u = get_uint();
        let k = get_uint();
        nodes[u].edge_size = k;
        for _ in 0..k {
            let to = get_uint();
            let cost = get_uint();
            nodes[u].edge_to.push(to);
            nodes[u].edge_cost.push(cost);
        }
    }

    dijkstra_noheap(&mut nodes);

    for i in 0..n {
        put_uint(i);
        print!(" ");
        put_uint(nodes[i].cost as usize);
        println!();
    }
}
