use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    tgt: i32,
    cost: i32, // kept for semantic parity; unused by algorithm
}

struct AdjacencyList {
    // In C this is an array of linked lists; here we store per-vertex outgoing edges.
    src: Vec<Vec<Edge>>,
    num_vertex: i32,
}

fn make_adjcency_list(v: i32) -> AdjacencyList {
    // C allocates V edge_list structs and initializes them empty.
    // We mirror that with V empty Vecs.
    let vv = if v < 0 { 0 } else { v as usize };
    AdjacencyList {
        src: vec![Vec::new(); vv],
        num_vertex: v,
    }
}

fn push_edge(graph: &mut AdjacencyList, source: i32, edge: Edge) {
    // C assumes `source` is a valid index; out-of-bounds would be UB.
    // We preserve that assumption and index directly (will panic on invalid input).
    graph.src[source as usize].push(edge);
}

fn is_cyclic(g: &AdjacencyList) -> i32 {
    let v = g.num_vertex;
    let vu = if v < 0 { 0 } else { v as usize };

    // C uses `int* in` initialized to 0.
    let mut in_deg: Vec<i32> = vec![0; vu];

    // Compute indegrees.
    for i in 0..vu {
        for e in &g.src[i] {
            in_deg[e.tgt as usize] += 1;
        }
    }

    // Kahn-like elimination; if at some step no zero-indegree vertex exists => cycle.
    for _j in 0..vu {
        let mut flag = 1;
        for i in 0..vu {
            if in_deg[i] == 0 {
                in_deg[i] = -1;
                flag = 0;
                for e in &g.src[i] {
                    in_deg[e.tgt as usize] -= 1;
                }
                break;
            }
        }
        if flag != 0 {
            return 1;
        }
    }
    0
}

fn main() {
    // Read all input, parse as ints like scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: i32 = it.next().unwrap().parse().unwrap();
    let e: i32 = it.next().unwrap().parse().unwrap();

    let mut graph = make_adjcency_list(v);

    for _ in 0..(e as usize) {
        let s: i32 = it.next().unwrap().parse().unwrap();
        let t: i32 = it.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        push_edge(&mut graph, s, edge);
    }

    let res = is_cyclic(&graph);
    print!("{}\n", res);
}
