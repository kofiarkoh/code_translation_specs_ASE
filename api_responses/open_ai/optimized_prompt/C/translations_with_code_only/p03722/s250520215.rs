use std::io::{self, BufRead};

const SUP: i64 = 1 << 60;

#[derive(Debug)]
struct Edge {
    t: usize,
    h: usize,
    cost: i64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let u: usize = nums.next().unwrap().parse().unwrap();
        let w: usize = nums.next().unwrap().parse().unwrap();
        let c: i64 = nums.next().unwrap().parse().unwrap();
        edges.push(Edge { t: u, h: w, cost: -c });
    }

    let mut min = vec![SUP; n + 1];
    min[1] = 0;

    for _ in 1..n {
        for edge in &edges {
            let u = edge.t;
            let w = edge.h;
            let c = edge.cost;
            if min[u] < SUP && min[w] > min[u] + c {
                min[w] = min[u] + c;
            }
        }
    }

    let ans = min[n];

    for _ in 1..=n {
        for edge in &edges {
            let u = edge.t;
            let w = edge.h;
            let c = edge.cost;
            if min[u] < SUP && min[w] > min[u] + c {
                min[w] = min[u] + c;
            }
        }
    }

    if min[n] == ans {
        println!("{}", -ans);
    } else {
        println!("inf");
    }
}
