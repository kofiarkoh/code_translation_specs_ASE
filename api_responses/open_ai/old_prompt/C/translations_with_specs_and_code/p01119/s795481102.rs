// AOJ 1634 Balance Scale
// Converted from C to Rust

use std::collections::HashSet;
use std::io::{self, BufRead};

const HASHSIZ: usize = 1000003;
const INF: i32 = 0x7fffffff;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(Ok(line)) = input.next() {
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        
        let m: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
        let mut a: Vec<i32> = Vec::with_capacity(n);
        let mut w: Vec<i32> = Vec::with_capacity(m);
        
        for _ in 0..n {
            let weight: i32 = input.next().unwrap().unwrap().trim().parse().unwrap();
            a.push(weight);
        }
        
        for _ in 0..m {
            let weight: i32 = input.next().unwrap().unwrap().trim().parse().unwrap();
            w.push(weight);
        }

        let mut q: Vec<i32> = vec![0; 60000]; // 3^10
        let mut end = 1;

        for &weight in &w {
            let e = end;
            for j in 0..e {
                q[end] = q[j] + weight;
                end += 1;
                q[end] = q[j] - weight;
                end += 1;
            }
        }

        let mut ha1: HashSet<i32> = HashSet::new();
        for &value in &q[..end] {
            if value > 0 {
                ha1.insert(value);
            }
        }

        let mut ans = 0;
        let mut found = false;

        for &target in &a {
            if !ha1.contains(&target) {
                if !found {
                    let mut temp_ans = INF;
                    let mut ha2: HashSet<i32> = HashSet::new();
                    for &value in &q[..end] {
                        let t = (target - value).abs();
                        if t < temp_ans {
                            temp_ans = t;
                        }
                        ha2.insert(t);
                    }
                    ans = temp_ans;
                    found = true;
                } else {
                    let mut temp_ans = INF;
                    let mut found_match = false;
                    for &value in &q[..end] {
                        let t = (target - value).abs();
                        if ha2.contains(&t) {
                            if t < temp_ans {
                                temp_ans = t;
                            }
                            found_match = true;
                        }
                    }
                    if !found_match {
                        ans = -1;
                        break;
                    } else {
                        ans = temp_ans;
                    }
                }
            }
        }

        println!("{}", if ans == INF { -1 } else { ans });
    }
}
