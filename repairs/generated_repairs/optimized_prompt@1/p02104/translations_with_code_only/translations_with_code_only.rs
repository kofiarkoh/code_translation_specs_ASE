// AOJ 3006: Chairs
// 2017.10.4 bal4u@uu

use std::io::{self, BufRead, Read};

const MAX: usize = 100003;

fn in_number() -> usize {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap();
    
    while c >= b'0' {
        n = (n << 3) + (n << 1) + (c - b'0') as usize;
        c = handle.bytes().next().unwrap().unwrap();
    }
    
    n
}

fn main() {
    let n = in_number();
    let mut p = vec![0; MAX];
    let mut sp = vec![0; MAX];
    let mut top = 0;
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); MAX];
    let mut hi = vec![0; MAX];
    let mut ans = vec![0; MAX];

    for i in 1..=n {
        let a = in_number();
        p[i] = a;
        hi[a] += 1;
    }

    for i in 1..=n {
        q[i] = vec![0; hi[i]];
    }

    hi.fill(0); // Reset hi to zero

    for i in 1..=n {
        let a = p[i];
        q[a][hi[a]] = i;
        hi[a] += 1;
    }

    for _ in 0..2 {
        for i in 1..=n {
            while hi[i] > 0 {
                top += 1;
                sp[top] = q[i][hi[i] - 1];
                hi[i] -= 1;
            }
            if ans[i] == 0 && top > 0 {
                ans[i] = sp[top];
                top -= 1;
            }
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
