use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency list for an undirected graph, vertices labeled 1..=N.
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        g[a].push(b);
        g[b].push(a);
    }

    // Predecessor array:
    // -1 = unreached, 0 for vertex 1, otherwise predecessor vertex id.
    let mut pred: Vec<i32> = vec![-1; n + 1];
    pred[1] = 0;

    // BFS queue; each vertex enqueued at most once.
    let mut que: Vec<usize> = Vec::with_capacity(n);
    let mut l: usize = 0;
    que.push(1);

    while l < que.len() {
        let i = que[l];
        l += 1;
        for &j in &g[i] {
            if pred[j] == -1 {
                pred[j] = i as i32;
                que.push(j);
            }
        }
    }

    // Output format must match C: "Yes" then N-1 lines for vertices 2..=N.
    let mut out = String::new();
    out.push_str("Yes\n");
    for i in 2..=n {
        out.push_str(&format!("{}\n", pred[i]));
    }
    print!("{out}");
}
