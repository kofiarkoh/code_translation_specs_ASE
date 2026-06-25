use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    tgt: i32,
    cost: i32, // kept for semantic parity; unused in algorithm
}

struct Graph {
    // adjacency list: for each source vertex, a list of outgoing edges
    src: Vec<Vec<Edge>>,
    num_vertex: i32,
}

fn make_adjcency_list(v: i32) -> Graph {
    // C allocates arrays sized by V; if V <= 0, behavior is dubious.
    // We emulate the practical intent: create an empty graph for non-positive V.
    let n = if v > 0 { v as usize } else { 0 };
    Graph {
        src: vec![Vec::new(); n],
        num_vertex: v,
    }
}

fn push_edge(graph: &mut Graph, source: i32, edge: Edge) {
    // C code assumes `source` is a valid index; otherwise it would be UB.
    // Here we index directly; if invalid, Rust will panic (closest practical behavior).
    graph.src[source as usize].push(edge);
}

fn is_cyclic(g: &Graph) -> i32 {
    let v_i32 = g.num_vertex;
    let v = if v_i32 > 0 { v_i32 as usize } else { 0 };

    let mut in_deg = vec![0i32; v];

    // compute in-degrees
    for i in 0..v {
        for e in &g.src[i] {
            // C assumes tgt is within [0, V); otherwise it would write out of bounds (UB).
            // We index directly; invalid input will panic.
            in_deg[e.tgt as usize] += 1;
        }
    }

    // Kahn-like process: if at some step no vertex has in-degree 0, cycle exists.
    for _j in 0..v {
        let mut flag = 1i32;
        for i in 0..v {
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
    // Read entire stdin and parse integers like scanf("%d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // no input; C would likely block, but for empty input just exit
    };
    let e_cnt: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut graph = make_adjcency_list(v);

    for _ in 0..e_cnt {
        let s: i32 = it.next().unwrap().parse().unwrap();
        let t: i32 = it.next().unwrap().parse().unwrap();
        let e = Edge { tgt: t, cost: 0 };
        push_edge(&mut graph, s, e);
    }

    let res = is_cyclic(&graph);
    print!("{}\n", res);
}
