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
        let src = vec![EdgeList {
            front: None,
            back: std::ptr::null_mut(),
            num_edge: 0,
        }; num_vertex];

        AdjacencyList {
            src,
            num_vertex,
        }
    }

    fn push_edge(&mut self, source: usize, edge: Edge) {
        let new_node = Box::new(EdgeListNode {
            edge,
            next: None,
        });

        let edge_list = &mut self.src[source];

        if edge_list.front.is_none() {
            edge_list.front = Some(new_node);
            edge_list.back = Box::into_raw(edge_list.front.as_ref().unwrap().as_ref() as *const _ as *mut _);
        } else {
            let back_node = unsafe { &mut *edge_list.back.unwrap() };
            back_node.next = Some(new_node);
            edge_list.back = Box::into_raw(back_node.next.as_ref().unwrap().as_ref() as *const _ as *mut _);
        }
        edge_list.num_edge += 1;
    }
}

fn is_cyclic(graph: &AdjacencyList) -> i32 {
    let mut in_degree = vec![0; graph.num_vertex];

    // Calculate in-degrees
    for edge_list in &graph.src {
        let mut current = edge_list.front.as_ref();
        while let Some(node) = current {
            in_degree[node.edge.tgt] += 1;
            current = node.next.as_ref();
        }
    }

    for _ in 0..graph.num_vertex {
        let mut found = false;

        for i in 0..graph.num_vertex {
            if in_degree[i] == 0 {
                in_degree[i] = -1; // Mark as visited
                found = true;

                let mut current = graph.src[i].front.as_ref();
                while let Some(node) = current {
                    in_degree[node.edge.tgt] -= 1;
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
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s: usize = parts.next().unwrap().parse().unwrap();
        let t: usize = parts.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        graph.push_edge(s, edge);
    }

    let result = is_cyclic(&graph);
    println!("{}", result);
}
