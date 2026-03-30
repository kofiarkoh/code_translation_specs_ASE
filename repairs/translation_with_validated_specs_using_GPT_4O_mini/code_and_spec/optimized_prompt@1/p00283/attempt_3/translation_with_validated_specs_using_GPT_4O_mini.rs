use std::io::{self, BufRead};

const INF: usize = 1000;
const MAX_ENERGY: usize = 100;
const MAX_ITEMS: usize = 100;

fn search(tkrk: usize, kkt: usize, N: usize, a: &[usize], e: &[usize], r: &[usize], memo: &mut [[isize; MAX_ENERGY + 1]]) -> isize {
    if tkrk <= 0 {
        return 0; // No items needed to reach target
    }
    if kkt > MAX_ENERGY {
        return INF as isize; // Exceeded energy limit
    }
    if memo[tkrk][kkt] > 0 {
        return memo[tkrk][kkt] - 1; // Return cached result
    }
    
    memo[tkrk][kkt] = INF as isize + 1; // Mark as uncomputed
    let mut ret = INF as isize;

    for i in 0..N {
        if kkt >= r[i] {
            let next_kkt = if kkt + e[i] > MAX_ENERGY { MAX_ENERGY } else { kkt + e[i] };
            let candidate = search(tkrk.saturating_sub(a[i]), next_kkt, N, a, e, r, memo) + 1;
            if candidate < ret {
                ret = candidate;
            }
        }
    }
    
    memo[tkrk][kkt] = ret + 1; // Store computed result
    ret
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let D: usize = parts.next().unwrap().parse().unwrap();
        let N: usize = parts.next().unwrap().parse().unwrap();

        if D == 0 && N == 0 {
            break; // Terminate on zero input
        }

        let mut a = vec![0; MAX_ITEMS];
        let mut e = vec![0; MAX_ITEMS];
        let mut r = vec![0; MAX_ITEMS];
        
        for i in 0..N {
            let item_line = lines.next().unwrap().unwrap();
            let mut item_parts = item_line.split_whitespace();
            a[i] = item_parts.next().unwrap().parse().unwrap();
            e[i] = item_parts.next().unwrap().parse().unwrap();
            r[i] = item_parts.next().unwrap().parse().unwrap();
        }

        let mut memo = [[0; MAX_ENERGY + 1]; MAX_ITEMS + 1]; // Adjust memo size to MAX_ITEMS + 1
        let ret = search(D, 0, N, &a, &e, &r, &mut memo);
        
        if ret == INF as isize {
            println!("NA");
        } else {
            println!("{}", ret);
        }
    }
}
