use std::io::{self, BufRead};
use std::ptr;

#[derive(Debug)]
struct Edge {
    tgt: usize,
    cost: usize,
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
        let src = vec![EdgeList {
            front: None,
            back: ptr::null_mut(),
            num_edge: 0,
        }; num_vertex];

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
                (*self.src[source].back).next = Some(Box::from_raw(new_node_ptr));
            }
        }
        self.src[source].back = new_node_ptr;
        self.src[source].num_edge += 1;
    }
}

fn is_cyclic(graph: &AdjacencyList) -> i32 {
    let mut in_degree = vec![0; graph.num_vertex];

    // Calculate in-degrees
    for i in 0..graph.num_vertex {
        let mut current = graph.src[i].front.as_ref();
        while let Some(node) = current {
            in_degree[node.edge.tgt] += 1;
            current = node.next.as_ref();
        }
    }

    for _ in 0..graph.num_vertex {
        let mut found = false;

        for i in 0..graph.num_vertex {
            if in_degree[i] == 0 {
                in_degree[i] = usize::MAX; // Mark as visited
                found = true;

                let mut current = graph.src[i].front.as_ref();
                while let Some(node) = current {
                    in_degree[node.edge.tgt] = in_degree[node.edge.tgt].saturating_sub(1);
                    current = node.next.as_ref();
                }
                break;
            }
        }

        if !found {
            return 1; // Cycle detected
        }
    }
    0 // No cycle detected
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
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_parts = edge_line.split_whitespace();
        let s: usize = edge_parts.next().unwrap().parse().unwrap();
        let t: usize = edge_parts.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        graph.push_edge(s, edge);
    }

    let result = is_cyclic(&graph);
    println!("{}", result);
}
