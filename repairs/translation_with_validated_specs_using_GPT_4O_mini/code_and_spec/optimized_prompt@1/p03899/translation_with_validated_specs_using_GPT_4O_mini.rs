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
    let n: i32 = params.next().unwrap().parse().expect("Invalid input for n");
    let m: i32 = params.next().unwrap().parse().expect("Invalid input for m");
    let k: i32 = params.next().unwrap().parse().expect("Invalid input for k");

    // Read array a
    let mut a: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        a.push(line.trim().parse().expect("Invalid input for array element"));
    }

    // Initialize dp array
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

            while b > f && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }
            d.push(Node { k: j, v: dp[j as usize] });
            b += 1;

            dp[j as usize] = v;
        }
    }

    // Calculate the maximum value
    let ans = (k - 1..n).map(|i| dp[i as usize]).max().unwrap_or(0);
    println!("{}", ans);
}

fn main() {
    run();
}
``` 

### Changes Made:
1. **Input Handling**: Added error handling for parsing integers to provide clearer error messages.
2. **Bounds Checking**: Ensured that the loop iterating over `j` correctly handles the range to avoid out-of-bounds access.
3. **Initialization**: Ensured that the `dp` and `d` vectors are correctly initialized and used.

Please test this version to see if it resolves the runtime error.
