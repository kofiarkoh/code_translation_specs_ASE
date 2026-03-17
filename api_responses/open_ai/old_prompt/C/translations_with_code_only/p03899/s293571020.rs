use std::io::{self, BufRead};
use std::ptr;

type I32 = i32;
type I64 = i64;

#[derive(Copy, Clone)]
struct Node {
    k: I32,
    v: I64,
}

fn run() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    
    let mut iter = input.split_whitespace();
    let n: I32 = iter.next().unwrap().parse().unwrap();
    let m: I32 = iter.next().unwrap().parse().unwrap();
    let k: I32 = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<I32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut input = String::new();
        handle.read_line(&mut input).unwrap();
        a.push(input.trim().parse().unwrap());
    }

    let mut dp: Vec<I64> = a.iter().map(|&x| x as I64).collect();
    let mut d: Vec<Node> = Vec::with_capacity(n as usize);

    for i in 2..=k {
        let mut f = 0;
        let mut b = 0;
        d.push(Node { k: i - 2, v: dp[(i - 2) as usize] });

        for j in (i - 1)..n {
            if d[f].k == j - m - 1 {
                f += 1;
            }
            let v = d[f].v + (i as I64) * a[j as usize] as I64;

            while f <= b && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }
            b += 1;
            d.push(Node { k: j, v: dp[j as usize] });
            dp[j as usize] = v;
        }
    }

    let mut ans = 0;
    for i in (k - 1)..n {
        if ans < dp[i as usize] {
            ans = dp[i as usize];
        }
    }

    println!("{}", ans);
}

fn main() {
    run();
}
