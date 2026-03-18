use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    tgt: usize,
    cost: i32, // kept for parity with C, though unused in cycle detection
}

struct AdjacencyList {
    src: Vec<Vec<Edge>>,
    num_vertex: usize,
}

fn make_adjcency_list(num_vertex: usize) -> AdjacencyList {
    AdjacencyList {
        src: vec![Vec::new(); num_vertex],
        num_vertex,
    }
}

fn push_edge(graph: &mut AdjacencyList, source: usize, edge: Edge) {
    // Per spec/C: invalid indices are not handled (would be UB in C).
    // In Rust, this will panic on out-of-bounds, which is the closest analogue.
    graph.src[source].push(edge);
}

fn is_cyclic(g: &AdjacencyList) -> i32 {
    let v = g.num_vertex;
    let mut indeg = vec![0i32; v];

    // Compute indegree
    for i in 0..v {
        for e in &g.src[i] {
            indeg[e.tgt] += 1;
        }
    }

    // Kahn-like elimination; if at some step no indegree-0 vertex exists, cycle exists.
    for _ in 0..v {
        let mut found = false;

        for i in 0..v {
            if indeg[i] == 0 {
                indeg[i] = -1; // mark removed
                found = true;
                for e in &g.src[i] {
                    indeg[e.tgt] -= 1;
                }
                break; // match C behavior: remove at most one vertex per outer iteration
            }
        }

        if !found {
            return 1;
        }
    }
    0
}

fn main() {
    // Read all input (scanf-like whitespace handling)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();

    let mut graph = make_adjcency_list(v);

    for _ in 0..e {
        let s: usize = it.next().unwrap().parse().unwrap();
        let t: usize = it.next().unwrap().parse().unwrap();
        let edge = Edge { tgt: t, cost: 0 };
        push_edge(&mut graph, s, edge);
    }

    let res = is_cyclic(&graph);
    print!("{res}\n");
}
