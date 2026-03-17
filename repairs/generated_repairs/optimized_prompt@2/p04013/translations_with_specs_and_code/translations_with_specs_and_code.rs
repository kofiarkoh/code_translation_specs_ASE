use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let a: i64 = iter.next().unwrap().parse().expect("Invalid input for A");

    let mut x = vec![0; n + 1];
    let mut sum = 0;

    for i in 1..=n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Invalid input for sequence element");
        sum += x[i];
    }

    let max_sum = sum as usize;
    let mut dp = vec![vec![vec![0; max_sum + 1]; n + 1]; n + 1];

    for j in 0..=n {
        for k in 0..=n {
            for s in 0..=max_sum {
                if j >= 1 && s < x[j] {
                    dp[j][k][s] = dp[j - 1][k][s];
                } else if j >= 1 && k >= 1 && s >= x[j] {
                    dp[j][k][s] = dp[j - 1][k][s] + dp[j - 1][k - 1][s - x[j]];
                } else if j == 0 && k == 0 && s == 0 {
                    dp[j][k][s] = 1;
                } else {
                    dp[j][k][s] = 0;
                }
            }
        }
    }

    let max_k = if sum / a < n as i64 { (sum / a) as usize } else { n };
    let mut ans = 0;

    for k in 1..=max_k {
        ans += dp[n][k][(k * a) as usize];
    }

    println!("{}", ans);
}
