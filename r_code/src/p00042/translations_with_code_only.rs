use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut count = 0;

    loop {
        let mut input = String::new();
        if let Err(_) = lines.next().unwrap().map(|line| line.trim().parse::<i32>()).and_then(|w| {
            if w == 0 {
                return Err(());
            }
            input.push_str(&format!("{}\n", w));
            Ok(w)
        }) {
            break;
        }

        let w: i32 = input.trim().parse().unwrap();
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut dp = vec![vec![-1; (w + 1) as usize]; (n + 1) as usize];
        let mut coin = vec![0; (n + 1) as usize];
        let mut wg = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split(',').collect();
            coin[i] = parts[0].trim().parse().unwrap();
            wg[i] = parts[1].trim().parse().unwrap();
        }

        dp[0][0] = 0;

        for i in 1..=n {
            dp[i][0] = 0;
            if dp[i][wg[i] as usize] < coin[i] {
                dp[i][wg[i] as usize] = coin[i];
            }
            for j in i..=n {
                dp[j][wg[i] as usize] = dp[i][wg[i] as usize];
            }

            for j in 1..=w {
                if dp[i - 1][j as usize] > 0 && dp[i][(j + wg[i]) as usize] < (coin[i] + dp[i - 1][j as usize]) {
                    dp[i][(j + wg[i]) as usize] = coin[i] + dp[i - 1][j as usize];
                    for k in i..=n {
                        dp[k][(j + wg[i]) as usize] = dp[i][(j + wg[i]) as usize];
                    }
                }
            }
        }

        let mut max_coin = 0;
        let mut weg = 0;

        for i in 0..=n {
            for j in 0..=w {
                if max_coin < dp[i][j as usize] {
                    max_coin = dp[i][j as usize];
                    weg = j;
                } else if max_coin == dp[i][j as usize] {
                    if weg >= j {
                        weg = j;
                    }
                }
            }
        }

        count += 1;
        println!("Case {}:\n{}\n{}", count, max_coin, weg);
    }
}
