use std::io::{self, BufRead};

const INF: usize = 1000;

fn search(tkrk: usize, kkt: usize, N: usize, a: &[usize], e: &[usize], r: &[usize], memo: &mut Vec<Vec<usize>>) -> usize {
    if tkrk <= 0 {
        return 0;
    }
    if memo[tkrk][kkt] > 0 {
        return memo[tkrk][kkt] - 1;
    }
    memo[tkrk][kkt] = INF + 1;

    let mut ret = INF;
    for i in 0..N {
        if kkt >= r[i] {
            let next_kkt = if kkt + e[i] > 100 { 100 } else { kkt + e[i] };
            let candidate = search(tkrk - a[i], next_kkt, N, a, e, r, memo) + 1;
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
        let D: usize = parts.next().unwrap().parse().unwrap();
        let N: usize = parts.next().unwrap().parse().unwrap();

        if D == 0 && N == 0 {
            break;
        }

        let mut a = vec![0; N];
        let mut e = vec![0; N];
        let mut r = vec![0; N];

        for i in 0..N {
            let line = reader.lines().next().unwrap().unwrap();
            let mut item_parts = line.split_whitespace();
            a[i] = item_parts.next().unwrap().parse().unwrap();
            e[i] = item_parts.next().unwrap().parse().unwrap();
            r[i] = item_parts.next().unwrap().parse().unwrap();
        }

        let mut memo = vec![vec![0; 101]; 101];
        let ret = search(D, 0, N, &a, &e, &r, &mut memo);
        
        if ret == INF {
            println!("NA");
        } else {
            println!("{}", ret);
        }
    }
}
