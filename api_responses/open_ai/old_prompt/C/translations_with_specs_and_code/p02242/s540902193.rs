use std::io::{self, BufRead, Write};

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

        if next.is_none() {
            break;
        }

        let next_node = next.unwrap();
        next_node.visited = true;
        let u = next_node.clone();

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
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut nodes = vec![Node {
        edge_to: Vec::new(),
        edge_cost: Vec::new(),
        edge_size: 0,
        cost: -1,
        visited: false,
    }; n];

    for _ in 0..n {
        let u: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
        nodes[u].edge_size = k;

        for _ in 0..k {
            let to: usize = lines.next().unwrap().unwrap().parse().unwrap();
            let cost: usize = lines.next().unwrap().unwrap().parse().unwrap();
            nodes[u].edge_to.push(to);
            nodes[u].edge_cost.push(cost);
        }
    }

    dijkstra_noheap(&mut nodes);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for i in 0..n {
        write!(handle, "{} {}", i, nodes[i].cost).unwrap();
        writeln!(handle).unwrap();
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic for Rust. The input is parsed using `parse()` to convert strings to integers.
2. **Node Structure**: The `Node` struct is defined with `Vec<usize>` for dynamic edge storage, which is more idiomatic than fixed-size arrays in Rust.
3. **Cost Representation**: The cost is represented as `isize` to allow for the value `-1` to indicate unreachable nodes, consistent with the C code.
4. **Loop Logic**: The loop for finding the next node to process is implemented using an `Option` to handle the case where no unvisited nodes are found.
5. **Output Handling**: The output is formatted to match the specification, ensuring that each node's index and cost are printed correctly.
6. **Cloning**: The `Node` is cloned when needed to avoid borrowing issues, which is a common practice in Rust to manage ownership and lifetimes.
