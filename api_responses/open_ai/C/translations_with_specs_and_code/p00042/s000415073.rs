use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut case_number = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let weight_limit: i32 = line.trim().parse().unwrap();
        if weight_limit == 0 {
            break; // Termination condition
        }

        let mut case_coins = String::new();
        reader.read_line(&mut case_coins).unwrap();
        let num_coin_types: usize = case_coins.trim().parse().unwrap();

        let mut weights = vec![0; num_coin_types + 1];
        let mut values = vec![0; num_coin_types + 1];
        let mut dp = vec![vec![-1; (weight_limit + 1) as usize]; num_coin_types + 1];

        for i in 1..=num_coin_types {
            let mut coin_input = String::new();
            reader.read_line(&mut coin_input).unwrap();
            let parts: Vec<i32> = coin_input
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            values[i] = parts[0];
            weights[i] = parts[1];
        }

        dp[0][0] = 0;

        for i in 1..=num_coin_types {
            dp[i][0] = 0;
            if dp[i][weights[i] as usize] < values[i] {
                dp[i][weights[i] as usize] = values[i];
            }
            for j in i..=num_coin_types {
                dp[j][weights[i] as usize] = dp[i][weights[i] as usize];
            }

            for j in 1..=weight_limit {
                if dp[i - 1][j as usize] > 0 && dp[i][(j + weights[i]) as usize] < (values[i] + dp[i - 1][j as usize]) {
                    dp[i][(j + weights[i]) as usize] = values[i] + dp[i - 1][j as usize];
                    for k in i..=num_coin_types {
                        dp[k][(j + weights[i]) as usize] = dp[i][(j + weights[i]) as usize];
                    }
                }
            }
        }

        let mut max_coin = 0;
        let mut total_weight = 0;

        for i in 0..=num_coin_types {
            for j in 0..=weight_limit {
                if max_coin < dp[i][j as usize] {
                    max_coin = dp[i][j as usize];
                    total_weight = j;
                } else if max_coin == dp[i][j as usize] {
                    if total_weight >= j {
                        total_weight = j;
                    }
                }
            }
        }

        case_number += 1;
        println!("Case {}:", case_number);
        println!("{}", max_coin);
        println!("{}", total_weight);
    }
}
