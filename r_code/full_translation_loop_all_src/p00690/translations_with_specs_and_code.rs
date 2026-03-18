use std::io::{self, Read};

#[derive(Clone, Copy)]
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
    tmp: &mut [i32],
    ans: &mut [i32],
    len: &mut i32,
    num: &mut usize,
) {
    // Try to extend the path from current station `a` to any other station `b`
    // using any unused edge (each edge can be used at most once).
    for b in 1..=n {
        if b == a {
            continue;
        }
        for i in 0..edges.len() {
            let e = edges[i];
            let connects = (e.a == a && e.b == b) || (e.a == b && e.b == a);
            if connects && !e.used {
                edges[i].used = true;
                tmp[k] = b;
                combi(k + 1, b, w + e.d, n, edges, tmp, ans, len, num);
                edges[i].used = false;
            }
        }
    }

    // Update best answer if current total weight is larger.
    if w > *len {
        *num = k;
        *len = w;
        ans[..*num].copy_from_slice(&tmp[..*num]);
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
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
        let m: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };

        // Spec: if number of stations is zero, terminate.
        if n == 0 {
            break;
        }

        // Read edges; spec says invalid connections are ignored.
        let mut edges: Vec<Edge> = Vec::new();
        for _ in 0..m {
            let a: i32 = it.next().unwrap().parse().unwrap();
            let b: i32 = it.next().unwrap().parse().unwrap();
            let d: i32 = it.next().unwrap().parse().unwrap();
            if (1..=n).contains(&a) && (1..=n).contains(&b) {
                edges.push(Edge {
                    a,
                    b,
                    d,
                    used: false,
                });
            }
        }

        // The original C code uses fixed-size arrays (ans/tmp length 12) and tbl[25].
        // Here we allocate enough for any simple path length in terms of vertices:
        // at most edges.len() + 1 vertices.
        let max_vertices = edges.len() + 1;
        let mut ans = vec![0i32; max_vertices];
        let mut tmp = vec![0i32; max_vertices];

        let mut len: i32 = 0;
        let mut num: usize = 0;

        // Enumerate all possible starting directed pairs (a -> b) via an edge,
        // then DFS extending without reusing edges.
        for a in 1..=n {
            for b in 1..=n {
                if b == a {
                    continue;
                }
                for i in 0..edges.len() {
                    let e = edges[i];
                    let connects = (e.a == a && e.b == b) || (e.a == b && e.b == a);
                    if connects && !edges[i].used {
                        edges[i].used = true;
                        tmp[0] = a;
                        tmp[1] = b;
                        combi(2, b, e.d, n, &mut edges, &mut tmp, &mut ans, &mut len, &mut num);
                        edges[i].used = false;
                    }
                }
            }
        }

        out.push_str(&format!("{len}\n"));
        if num > 0 {
            out.push_str(&format!("{}", ans[0]));
            for i in 1..num {
                out.push_str(&format!(" {}", ans[i]));
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
