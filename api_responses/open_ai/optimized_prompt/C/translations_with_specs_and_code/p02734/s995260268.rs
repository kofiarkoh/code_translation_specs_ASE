use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let s: usize = iter.next().unwrap().parse().unwrap();

    let mut a = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        a.push(num.parse::<usize>().unwrap());
    }

    let mod_val = 998244353;
    let mut dp = vec![vec![vec![0; 2]; s + 1]; n];

    dp[0][0][0] = 1;
    dp[0][0][1] = 1;
    if a[0] <= s {
        dp[0][a[0]][0] = 1;
        dp[0][a[0]][1] = 1;
    }

    for i in 1..n {
        for j in 0..=s {
            dp[i][j][0] = dp[i - 1][j][0]; // Carry forward the previous count
            if j + a[i] <= s {
                dp[i][j + a[i]][0] = (dp[i][j + a[i]][0] + dp[i - 1][j][0]) % mod_val; // Update count for current element
            }
        }
        dp[i][0][0] = (dp[i][0][0] + 1) % mod_val; // Increment for the empty selection
        if a[i] <= s {
            dp[i][a[i]][0] = (dp[i][a[i]][0] + 1) % mod_val; // Increment for the current element
        }
        for j in 0..=s {
            dp[i][j][1] = (dp[i - 1][j][1] + dp[i][j][0]) % mod_val; // Update total ways
        }
    }

    println!("{}", dp[n - 1][s][1]);
}
