use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: i32,
    next: Option<usize>,
}

fn main() {
    // Read entire stdin, parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // C uses fixed-size arrays of size 1<<17. Keep the same capacity.
    const MAX: usize = 1 << 17;

    // d[] initialized to -1 for i=0..=N; others unused.
    let mut d = vec![-1i32; MAX];

    // Adjacency list: head index into `edges` (None == nil).
    let mut head: Vec<Option<usize>> = vec![None; MAX];
    let mut edges: Vec<Edge> = Vec::with_capacity(2 * m);

    // push(i, j): insert at head (like C malloc + pointer linking).
    let mut push = |i: usize, j: i32, head: &mut [Option<usize>], edges: &mut Vec<Edge>| {
        let idx = edges.len();
        edges.push(Edge { to: j, next: head[i] });
        head[i] = Some(idx);
    };

    for i in 0..=n {
        d[i] = -1;
        head[i] = None; // G[i].next = nil
    }

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        push(a, b as i32, &mut head, &mut edges);
        push(b, a as i32, &mut head, &mut edges);
    }

    // BFS with fixed-size queue like C.
    let mut que = vec![0usize; MAX];
    let mut l: usize = 0;
    let mut r: usize = 0;

    d[1] = 0;
    que[r] = 1;
    r += 1;

    while l != r {
        let i = que[l];
        l += 1;

        let mut x = head[i];
        while let Some(eidx) = x {
            let e = edges[eidx];
            let j = e.to as usize;
            if d[j] == -1 {
                d[j] = i as i32;
                que[r] = j;
                r += 1;
            }
            x = e.next;
        }
    }

    // Output exactly as C: "Yes\n" then parents for 2..=N each on its own line.
    let mut out = String::new();
    out.push_str("Yes\n");
    for i in 2..=n {
        out.push_str(&format!("{}\n", d[i]));
    }
    print!("{out}");
}
