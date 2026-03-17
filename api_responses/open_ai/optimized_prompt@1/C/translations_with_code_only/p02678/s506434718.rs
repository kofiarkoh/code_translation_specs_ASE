use std::io::{self, BufRead};

#[derive(Debug)]
struct Edge {
    to: usize,
    next: Option<Box<Edge>>,
}

const MAX_N: usize = 1 << 17;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    let mut d = vec![-1; n + 1];
    let mut g: Vec<Option<Box<Edge>>> = vec![None; n + 1];
    let mut que = vec![0; MAX_N];
    let mut l = 0;
    let mut r = 0;

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let a: usize = nums.next().unwrap().parse().unwrap();
        let b: usize = nums.next().unwrap().parse().unwrap();
        push(&mut g, a, b);
        push(&mut g, b, a);
    }

    d[1] = 0;
    que[r] = 1;
    r += 1;

    while l != r {
        let i = que[l];
        l += 1;
        let mut x = g[i].as_ref();

        while let Some(edge) = x {
            let j = edge.to;
            if d[j] == -1 {
                d[j] = i as isize; // Store the parent node
                que[r] = j;
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

fn push(g: &mut Vec<Option<Box<Edge>>>, i: usize, j: usize) {
    let new_edge = Box::new(Edge {
        to: j,
        next: g[i].take(),
    });
    g[i] = Some(new_edge);
}
