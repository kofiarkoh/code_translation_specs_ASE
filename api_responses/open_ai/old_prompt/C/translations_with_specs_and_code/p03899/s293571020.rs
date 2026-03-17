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
    let n: i32 = params.next().unwrap().parse().unwrap();
    let m: i32 = params.next().unwrap().parse().unwrap();
    let k: i32 = params.next().unwrap().parse().unwrap();

    // Preconditions
    assert!(n >= 0 && m >= 0 && k >= 0);
    assert!(m < n);

    // Allocate and read array a
    let mut a = vec![0; n as usize];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        a[i as usize] = line.trim().parse().unwrap();
    }

    // Allocate dp array
    let mut dp = a.clone().into_iter().map(|x| x as i64).collect::<Vec<i64>>();

    // Allocate d array
    let mut d = vec![Node { k: 0, v: 0 }; n as usize];

    for i in 2..=k {
        let mut f = 0;
        let mut b = 0;
        d[0] = Node { k: i - 2, v: dp[(i - 2) as usize] };

        for j in (i - 1)..n {
            if d[f].k == j - m - 1 {
                f += 1;
            }
            let v = d[f].v + (i as i64) * a[j as usize] as i64;

            while f <= b && d[b].v <= dp[j as usize] {
                b -= 1;
            }
            b += 1;
            d[b] = Node { k: j, v: dp[j as usize] };
            dp[j as usize] = v;
        }
    }

    // Compute the maximum value
    let ans = (k - 1..n).map(|i| dp[i as usize]).max().unwrap_or(0);
    println!("{}", ans);
}

fn main() {
    run();
}
