use std::io::{self, BufRead};

const INF: usize = 1000;
const MAX_TARGET: usize = 100;
const MAX_ITEMS: usize = 100;

fn search(tkrk: usize, kkt: usize, memo: &mut [[usize; MAX_TARGET + 1]; MAX_TARGET + 1], a: &[usize], e: &[usize], r: &[usize], n: usize) -> usize {
    if tkrk <= 0 {
        return 0;
    }
    if memo[tkrk][kkt] > 0 {
        return memo[tkrk][kkt] - 1;
    }
    memo[tkrk][kkt] = INF + 1;

    let mut ret = INF;
    for i in 0..n {
        if kkt >= r[i] {
            let next_kkt = if kkt + e[i] > MAX_TARGET { MAX_TARGET } else { kkt + e[i] };
            let candidate = search(tkrk - a[i], next_kkt, memo, a, e, r, n) + 1;
            if candidate < ret {
                ret = candidate;
            }
        }
    }
    memo[tkrk][kkt] = ret + 1;
    ret
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let d: usize = parts.next().unwrap().parse().unwrap();
        let n: usize = parts.next().unwrap().parse().unwrap();

        if d == 0 && n == 0 {
            break;
        }

        let mut a = vec![0; n];
        let mut e = vec![0; n];
        let mut r = vec![0; n];

        for i in 0..n {
            let line = reader.lines().next().unwrap();
            let line = line.unwrap(); // Unwrap the Result here
            let mut parts = line.split_whitespace();
            a[i] = parts.next().unwrap().parse().unwrap();
            e[i] = parts.next().unwrap().parse().unwrap();
            r[i] = parts.next().unwrap().parse().unwrap();
        }

        let mut memo = [[0; MAX_TARGET + 1]; MAX_TARGET + 1];
        let ret = search(d, 0, &mut memo, &a, &e, &r, n);
        
        if ret == INF {
            println!("NA");
        } else {
            println!("{}", ret);
        }
    }
}
