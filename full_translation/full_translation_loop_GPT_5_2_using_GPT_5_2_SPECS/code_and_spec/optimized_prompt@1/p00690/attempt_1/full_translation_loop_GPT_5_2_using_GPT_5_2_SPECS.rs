use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    a: i32,
    b: i32,
    d: i32,
    used: bool,
}

fn combi(
    k: usize,
    a: i32,
    w: i32,
    n: i32,
    edges: &mut [Edge],
    tmp: &mut [i32; 12],
    best_len: &mut i32,
    best_num: &mut usize,
    best_ans: &mut [i32; 12],
) {
    // Explore all possible next vertices b (1..=n), and for each unused edge (a,b) traverse it.
    for b in 1..=n {
        if b == a {
            continue;
        }
        for i in 0..edges.len() {
            let e = &edges[i];
            let connects = (e.a == a && e.b == b) || (e.a == b && e.b == a);
            if connects && !e.used {
                edges[i].used = true;
                tmp[k] = b;
                combi(k + 1, b, w + edges[i].d, n, edges, tmp, best_len, best_num, best_ans);
                edges[i].used = false;
            }
        }
    }

    // Update best if this trail is better.
    if w > *best_len {
        *best_len = w;
        *best_num = k;
        best_ans[..k].copy_from_slice(&tmp[..k]);
    }
}

fn main() {
    // Read all input as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut edges: Vec<Edge> = Vec::with_capacity(m);
        for _ in 0..m {
            let a: i32 = it.next().unwrap().parse().unwrap();
            let b: i32 = it.next().unwrap().parse().unwrap();
            let d: i32 = it.next().unwrap().parse().unwrap();
            edges.push(Edge { a, b, d, used: false });
        }

        let mut best_len: i32 = 0;
        let mut best_num: usize = 0;
        let mut best_ans = [0i32; 12];
        let mut tmp = [0i32; 12];

        // Match the C code's behavior: start by choosing an initial edge (a,b),
        // then recurse from b with that edge marked used.
        for a in 1..=n {
            for b in 1..=n {
                if b == a {
                    continue;
                }
                for i in 0..edges.len() {
                    let connects = (edges[i].a == a && edges[i].b == b) || (edges[i].a == b && edges[i].b == a);
                    if connects && !edges[i].used {
                        edges[i].used = true;
                        tmp[0] = a;
                        tmp[1] = b;
                        combi(
                            2,
                            b,
                            edges[i].d,
                            n,
                            &mut edges,
                            &mut tmp,
                            &mut best_len,
                            &mut best_num,
                            &mut best_ans,
                        );
                        edges[i].used = false;
                    }
                }
            }
        }

        out.push_str(&format!("{best_len}\n"));
        // Per spec, the vertex sequence length is at least 1 under preconditions.
        // The original C code assumes best_num >= 1 and prints ans[0] unconditionally.
        out.push_str(&format!("{}", best_ans[0]));
        for i in 1..best_num {
            out.push_str(&format!(" {}", best_ans[i]));
        }
        out.push('\n');
    }

    print!("{out}");
}
