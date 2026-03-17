// Aizu 1108: A Long Ride on a Railway
// Translated from C to Rust

use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Connection {
    a: usize,
    b: usize,
    d: usize,
    f: bool,
}

fn combi(k: usize, a: usize, w: usize, n: usize, m: usize, tbl: &mut [Connection], tmp: &mut Vec<usize>, ans: &mut Vec<usize>, len: &mut usize, num: &mut usize) {
    for b in 1..=n {
        if b == a {
            continue;
        }
        for i in 0..m {
            if (tbl[i].a == a && tbl[i].b == b || tbl[i].a == b && tbl[i].b == a) && !tbl[i].f {
                tbl[i].f = true;
                tmp.push(b);
                combi(k + 1, b, w + tbl[i].d, n, m, tbl, tmp, ans, len, num);
                tbl[i].f = false;
                tmp.pop(); // Remove last element after backtracking
            }
        }
    }
    if w > *len {
        *num = k;
        *len = w;
        *ans = tmp.clone(); // Update the answer with the current path
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut tbl: Vec<Connection> = vec![Connection { a: 0, b: 0, d: 0, f: false }; m];
        for i in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            tbl[i].a = parts.next().unwrap().parse().unwrap();
            tbl[i].b = parts.next().unwrap().parse().unwrap();
            tbl[i].d = parts.next().unwrap().parse().unwrap();
            tbl[i].f = false;
        }

        let mut ans: Vec<usize> = Vec::new();
        let mut tmp: Vec<usize> = Vec::new();
        let mut len = 0;
        let mut num = 0;

        for a in 1..=n {
            for b in 1..=n {
                if b == a {
                    continue;
                }
                for i in 0..m {
                    if (tbl[i].a == a && tbl[i].b == b || tbl[i].b == a && tbl[i].a == b) && !tbl[i].f {
                        tbl[i].f = true;
                        tmp.push(a);
                        tmp.push(b);
                        combi(2, b, tbl[i].d, n, m, &mut tbl, &mut tmp, &mut ans, &mut len, &mut num);
                        tbl[i].f = false;
                        tmp.pop(); // Remove last element after backtracking
                        tmp.pop(); // Remove the previous station as well
                    }
                }
            }
        }

        println!("{}", len);
        if !ans.is_empty() {
            print!("{}", ans[0]);
            for i in 1..num {
                print!(" {}", ans[i]);
            }
            println!();
        }
    }
}
