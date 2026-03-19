// AOJ GRL_4_B Path/Cycle - Topological Sort
// 2018.4.26 bal4u

use std::io::{self, BufRead};

const MAX_V: usize = 10002;

fn in_number() -> usize {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Changed to mutable
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn topological_sort(V: usize, to: &Vec<Vec<usize>>, hi: &Vec<usize>, count: &mut Vec<usize>) {
    let mut q = Vec::new();
    let mut top = 0;
    let mut end = 0;

    for i in 0..V {
        if count[i] == 0 {
            q.push(i);
            end += 1;
        }
    }

    while top < end {
        let i = q[top];
        top += 1;
        println!("{}", i);
        for &k in &to[i] {
            if count[k] > 0 {
                count[k] -= 1;
                if count[k] == 0 {
                    q.push(k);
                    end += 1;
                }
            }
        }
    }
}

fn main() {
    let V = in_number();
    let E = in_number();
    
    let mut hi = vec![0; V];
    let mut memo = Vec::with_capacity(E * 2);
    
    for _ in 0..E {
        let s = in_number();
        let t = in_number();
        memo.push(s);
        memo.push(t);
        hi[s] += 1;
    }

    let mut to: Vec<Vec<usize>> = vec![Vec::new(); V];
    let mut count = vec![0; V];

    for i in 0..V {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    hi.fill(0); // Reset hi for the next use
    let mut j = 0;
    while j < memo.len() {
        let s = memo[j];
        let t = memo[j + 1];
        to[s][hi[s]] = t;
        hi[s] += 1;
        count[t] += 1;
        j += 2;
    }

    topological_sort(V, &to, &hi, &mut count);
}
