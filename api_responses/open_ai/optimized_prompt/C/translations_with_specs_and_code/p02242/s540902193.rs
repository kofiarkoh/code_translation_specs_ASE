use std::io::{self, BufRead, Write};

#[derive(Debug)]
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
        let u = next_node.clone();

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
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut nodes: Vec<Node> = vec![Node {
        edge_to: Vec::new(),
        edge_cost: Vec::new(),
        edge_size: 0,
        cost: -1,
        visited: false,
    }; n];

    for _ in 0..n {
        let u: usize = input.next().unwrap().unwrap().parse().unwrap();
        let k: usize = input.next().unwrap().unwrap().parse().unwrap();
        nodes[u].edge_size = k;

        for _ in 0..k {
            let to: usize = input.next().unwrap().unwrap().parse().unwrap();
            let cost: i32 = input.next().unwrap().unwrap().parse().unwrap();
            nodes[u].edge_to.push(to);
            nodes[u].edge_cost.push(cost);
        }
    }

    dijkstra_noheap(&mut nodes);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, node) in nodes.iter().enumerate() {
        writeln!(handle, "{} {}", i, node.cost).unwrap();
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic for Rust. It reads lines and parses them into integers.
2. **Node Structure**: The `Node` struct is defined with `Vec` for dynamic edge storage, which is more idiomatic than fixed-size arrays in Rust.
3. **Dijkstra's Algorithm**: The algorithm is implemented in a similar manner to the C code, ensuring that the logic and flow are preserved.
4. **Output Handling**: The output is formatted using `writeln!`, which handles newlines automatically, ensuring the output format matches the specification.
5. **Error Handling**: The code assumes valid input as per the specification. If invalid input is provided, it will panic due to the unwrap calls. This behavior is consistent with the specification's undefined behavior for invalid inputs.
