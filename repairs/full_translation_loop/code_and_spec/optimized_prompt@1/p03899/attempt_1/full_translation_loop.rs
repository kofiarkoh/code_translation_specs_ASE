use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    k: i32,
    v: i64,
}

fn run() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines().flat_map(Result::ok);
    
    // Read n, m, k
    let first_line = input.next().unwrap();
    let mut params = first_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let n = params.next().unwrap();
    let m = params.next().unwrap();
    let k = params.next().unwrap();

    // Read the sequence of integers
    let a: Vec<i32> = input.by_ref().take(n as usize).map(|x| x.parse::<i32>().unwrap()).collect();

    // Initialize the dynamic programming array
    let mut dp = vec![0i64; n as usize];
    for i in 0..n {
        dp[i as usize] = a[i as usize] as i64;
    }

    // Initialize the node array
    let mut d: Vec<Node> = Vec::with_capacity(n as usize);
    
    for i in 2..=k {
        let mut f = 0;
        let mut b = 0;
        d.push(Node { k: i - 2, v: dp[(i - 2) as usize] });

        for j in (i - 1)..n {
            if d[f as usize].k == j - m - 1 {
                f += 1;
            }
            let v = d[f as usize].v + (i as i64) * (a[j as usize] as i64);
            while b > f && d[b as usize - 1].v <= dp[j as usize] {
                b -= 1;
            }
            d.push(Node { k: j, v: dp[j as usize] });
            dp[j as usize] = v;
            b += 1;
        }
    }

    // Calculate the maximum weighted sum
    let mut ans = 0;
    for i in (k - 1)..n {
        if ans < dp[i as usize] {
            ans = dp[i as usize];
        }
    }

    // Print the result
    println!("{}", ans);
}

fn main() {
    run();
}
