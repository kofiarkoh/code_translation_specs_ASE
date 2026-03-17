use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    k: i32,
    v: i64,
}

fn run() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n, m, k
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: i32 = params.next().unwrap().parse().expect("Failed to parse n");
    let m: i32 = params.next().unwrap().parse().expect("Failed to parse m");
    let k: i32 = params.next().unwrap().parse().expect("Failed to parse k");

    // Read the sequence of integers
    let mut a: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        a.push(line.trim().parse().expect("Failed to parse integer"));
    }

    // Initialize the dynamic programming array
    let mut dp: Vec<i64> = vec![0; n as usize];
    for (i, &x) in a.iter().enumerate() {
        dp[i] = x as i64;
    }
    let mut d: Vec<Node> = Vec::with_capacity(n as usize);

    for i in 2..=k {
        let mut f = 0;
        let mut b = 0;
        d.push(Node { k: i - 2, v: dp[(i - 2) as usize] });

        for j in (i - 1)..n {
            if f > 0 && d[f as usize].k == j - m - 1 {
                f += 1;
            }
            let v = d[f as usize].v + (i as i64) * (a[j as usize] as i64);
            while b > f && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }
            d.push(Node { k: j, v: dp[j as usize] });
            dp[j as usize] = v;
            b += 1;
        }
    }

    // Calculate the maximum weighted sum
    let ans = (k - 1..n).map(|i| dp[i as usize]).max().unwrap_or(0);
    println!("{}", ans);
}

fn main() {
    run();
}
