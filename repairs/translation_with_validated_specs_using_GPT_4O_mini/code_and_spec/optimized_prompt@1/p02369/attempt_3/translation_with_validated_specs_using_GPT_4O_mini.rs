use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    tgt: usize,
    cost: i32,
}

#[derive(Debug)]
struct EdgeListNode {
    edge: Edge,
    next: Option<Box<EdgeListNode>>,
}

#[derive(Debug)]
struct EdgeList {
    front: Option<Box<EdgeListNode>>,
    back: Option<*mut EdgeListNode>, // Using raw pointer for back to avoid ownership issues
    num_edge: usize,
}

#[derive(Debug)]
struct AdjacencyList {
    src: Vec<EdgeList>,
    num_vertex: usize,
}

impl AdjacencyList {
    fn new(num_vertex: usize) -> Self {
        let src = (0..num_vertex)
            .map(|_| EdgeList {
                front: None,
                back: None,
                num_edge: 0,
            })
            .collect();

        AdjacencyList { src, num_vertex }
    }

    fn push_edge(&mut self, source: usize, edge: Edge) {
        let new_node = Box::new(EdgeListNode {
            edge,
            next: None,
        });

        let new_node_ptr = Box::into_raw(new_node);

        if self.src[source].front.is_none() {
            self.src[source].front = unsafe { Some(Box::from_raw(new_node_ptr)) };
        } else {
            unsafe {
                (*self.src[source].back.unwrap()).next = Some(Box::from_raw(new_node_ptr));
            }
        }
        self.src[source].back = Some(new_node_ptr);
        self.src[source].num_edge += 1;
    }

    fn is_cyclic(&self) -> i32 {
        let mut in_degree = vec![0; self.num_vertex];

        // Calculate in-degrees
        for i in 0..self.num_vertex {
            let mut current = self.src[i].front.as_ref();
            while let Some(node) = current {
                in_degree[node.edge.tgt] += 1;
                current = node.next.as_ref();
            }
        }

        // Kahn's algorithm for cycle detection
        let mut queue = Vec::new();
        for i in 0..self.num_vertex {
            if in_degree[i] == 0 {
                queue.push(i);
            }
        }

        let mut count = 0;

        while !queue.is_empty() {
            let current = queue.remove(0);
            count += 1;

            let mut current_node = self.src[current].front.as_ref();
            while let Some(node) = current_node {
                in_degree[node.edge.tgt] -= 1;
                if in_degree[node.edge.tgt] == 0 {
                    queue.push(node.edge.tgt);
                }
                current_node = node.next.as_ref();
            }
        }

        if count == self.num_vertex {
            0 // No cycle
        } else {
            1 // Cycle exists
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let v: usize = parts.next().unwrap().parse().unwrap();
    let e: usize = parts.next().unwrap().parse().unwrap();

    let mut graph = AdjacencyList::new(v);

    for _ in 0..e {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s: usize = parts.next().unwrap().parse().unwrap();
        let t: usize = parts.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        graph.push_edge(s, edge);
    }

    let res = graph.is_cyclic();
    println!("{}", res);
}
