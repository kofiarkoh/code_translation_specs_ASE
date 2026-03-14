use std::io::{self, BufRead};

const INF: i32 = 100000;
const MAX_N: usize = 100;

fn dijkstra(r: &[[i32; MAX_N]; MAX_N], n: usize, start: usize, goal: usize) -> i32 {
    let mut d = [INF; MAX_N];
    let mut f = [0; MAX_N];

    d[start - 1] = 1;

    loop {
        let mut minimum = INF;
        let mut p = 0;

        for i in 0..n {
            if d[i] < minimum && f[i] == 0 {
                minimum = d[i];
                p = i;
            }
        }

        if minimum == INF {
            break;
        }
        f[p] = 1;

        for i in 0..n {
            if d[p] + r[p][i] < d[i] {
                d[i] = d[p] + r[p][i];
            }
        }
    }

    d[goal - 1]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut r = [[INF; MAX_N]; MAX_N];

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let r1: usize = parts.next().unwrap().parse().unwrap();
        let k1: usize = parts.next().unwrap().parse().unwrap();
        for _ in 0..k1 {
            let temp: usize = parts.next().unwrap().parse().unwrap();
            r[r1 - 1][temp - 1] = 1;
        }
    }

    let p: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..p {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s1: usize = parts.next().unwrap().parse().unwrap();
        let d1: usize = parts.next().unwrap().parse().unwrap();
        let v1: i32 = parts.next().unwrap().parse().unwrap();

        let temp = dijkstra(&r, n, s1, d1);
        if v1 - temp < 0 {
            println!("NA");
        } else {
            println!("{}", temp);
        }
    }
}
