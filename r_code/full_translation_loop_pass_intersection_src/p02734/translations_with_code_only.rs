use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 to mirror C's int/long arithmetic reasonably.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse::<i64>().unwrap() as usize,
        None => return,
    };
    let s_i64: i64 = it.next().unwrap().parse().unwrap();
    let s: usize = s_i64 as usize;

    let mut a: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let modu: i64 = 998_244_353;

    // dp[i][j][k] where k in {0,1}
    // Use i64 and apply modulo like the C code does.
    let mut dp = vec![vec![[0i64; 2]; s + 1]; n];

    dp[0][0][0] = 1;
    dp[0][0][1] = 1;
    if a[0] >= 0 && (a[0] as usize) <= s {
        let idx = a[0] as usize;
        dp[0][idx][0] = 1;
        dp[0][idx][1] = 1;
    }

    for i in 1..n {
        for j in 0..=s {
            dp[i][j][0] += dp[i - 1][j][0];
            // Note: the C code does not mod after this addition; we preserve that.
            let aj = a[i];
            if aj >= 0 {
                let jp = j as i64 + aj;
                if jp <= s as i64 {
                    let jp = jp as usize;
                    dp[i][jp][0] += dp[i - 1][j][0];
                    dp[i][jp][0] %= modu;
                }
            }
        }

        dp[i][0][0] += 1;
        if a[i] >= 0 && (a[i] as usize) <= s {
            let idx = a[i] as usize;
            dp[i][idx][0] += 1;
            dp[i][idx][0] %= modu;
        }

        for j in 0..=s {
            dp[i][j][1] = dp[i - 1][j][1] + dp[i][j][0];
            dp[i][j][1] %= modu;
        }
    }

    println!("{}", dp[n - 1][s][1]);
}
