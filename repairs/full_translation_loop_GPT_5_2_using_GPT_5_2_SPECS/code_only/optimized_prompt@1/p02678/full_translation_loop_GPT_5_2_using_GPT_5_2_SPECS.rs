use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    to: i32,
    next: i32, // index of next edge in adjacency list, -1 means nil
}

fn main() {
    // Read entire stdin, parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // no input
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // C uses fixed-size arrays of 1<<17; we mirror that capacity.
    const MAX: usize = 1 << 17;

    // d initialized to -1; later d[1]=0 and for visited nodes d[j]=parent i.
    let mut d = vec![-1i32; MAX];
    let mut head = vec![-1i32; MAX]; // G[i].next, -1 is nil

    // Queue array and indices l,r (no wraparound in C; assume fits).
    let mut que = vec![0i32; MAX];
    let mut l: usize = 0;
    let mut r: usize = 0;

    // Edge pool to emulate malloc'ed linked list nodes.
    let mut edges: Vec<Edge> = Vec::with_capacity(2 * m);

    // push(i, j): insert at head of adjacency list for i.
    let mut push = |i: usize, j: i32, head: &mut [i32], edges: &mut Vec<Edge>| {
        let next = head[i];
        edges.push(Edge { to: j, next });
        head[i] = (edges.len() as i32) - 1;
    };

    // Initialize for i=0..=N
    for i in 0..=n {
        d[i] = -1;
        head[i] = -1; // nil
    }

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        push(a, b as i32, &mut head, &mut edges);
        push(b, a as i32, &mut head, &mut edges);
    }

    d[1] = 0;
    que[r] = 1;
    r += 1;

    while l != r {
        let i = que[l] as usize;
        l += 1;

        let mut x = head[i];
        while x != -1 {
            let e = edges[x as usize];
            let j = e.to as usize;
            if d[j] == -1 {
                d[j] = i as i32;
                que[r] = j as i32;
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
