use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    k: i32,
    v: i64,
}

fn run() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    
    // Read the first line of input
    handle.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        input.clear();
        handle.read_line(&mut input).unwrap();
        a.push(input.trim().parse().unwrap());
    }

    let mut dp: Vec<i64> = a.iter().map(|&x| x as i64).collect();
    let mut d: Vec<Node> = Vec::with_capacity(n as usize);

    for i in 2..=k {
        let mut f = 0;
        let mut b = 0;
        d.push(Node { k: i - 2, v: dp[(i - 2) as usize] });

        for j in (i - 1)..n {
            if d[f].k == j - m - 1 {
                f += 1;
            }
            let v = d[f].v + (i as i64) * a[j as usize] as i64;

            while b >= f && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }
            d.push(Node { k: j, v: dp[j as usize] });
            b += 1;
            dp[j as usize] = v;
        }
    }

    let ans = dp.iter().skip((k - 1) as usize).cloned().fold(0, |acc, x| acc.max(x));
    println!("{}", ans);
}

fn main() {
    run();
}
