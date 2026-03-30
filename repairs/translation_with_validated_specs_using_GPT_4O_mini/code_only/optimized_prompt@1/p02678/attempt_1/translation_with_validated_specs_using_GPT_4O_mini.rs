use std::io::{self, BufRead};

struct Edge {
    to: usize,
    next: Option<Box<Edge>>,
}

const MAX_N: usize = 1 << 17;

fn push(graph: &mut Vec<Option<Box<Edge>>>, i: usize, j: usize) {
    let new_edge = Box::new(Edge {
        to: j,
        next: graph[i].take(),
    });
    graph[i] = Some(new_edge);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut d = vec![-1; n + 1];
    let mut graph: Vec<Option<Box<Edge>>> = vec![None; n + 1];
    
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        push(&mut graph, a, b);
        push(&mut graph, b, a);
    }

    d[1] = 0;
    let mut queue = vec![1];
    let mut l = 0;
    let mut r = 1;

    while l < r {
        let i = queue[l];
        l += 1;

        let mut x = graph[i].as_ref();
        while let Some(edge) = x {
            let j = edge.to;
            if d[j] == -1 {
                d[j] = i as isize; // Store the parent node
                queue.push(j);
                r += 1;
            }
            x = edge.next.as_ref();
        }
    }

    println!("Yes");
    for i in 2..=n {
        println!("{}", d[i]);
    }
}
