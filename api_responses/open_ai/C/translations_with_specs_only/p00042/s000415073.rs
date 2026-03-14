use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut case_number = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace();

        let weight_limit: usize = inputs.next().unwrap().parse().unwrap();
        if weight_limit == 0 {
            break; // Termination condition
        }

        let num_coin_types: usize = inputs.next().unwrap().parse().unwrap();
        let mut coins = Vec::new();

        for _ in 0..num_coin_types {
            let value: usize = inputs.next().unwrap().parse().unwrap();
            let weight: usize = inputs.next().unwrap().parse().unwrap();
            coins.push((value, weight));
        }

        let (max_value, total_weight) = knapsack(weight_limit, &coins);
        println!("Case {}: {} {}", case_number, max_value, total_weight);
        case_number += 1;
    }
}

fn knapsack(weight_limit: usize, coins: &[(usize, usize)]) -> (usize, usize) {
    let mut dp = vec![0; weight_limit + 1];
    let mut weight_used = vec![0; weight_limit + 1];

    for &(value, weight) in coins {
        for w in (weight..=weight_limit).rev() {
            let new_value = dp[w - weight] + value;
            if new_value > dp[w] {
                dp[w] = new_value;
                weight_used[w] = weight; // Track the weight used to achieve this value
            }
        }
    }

    // Find the maximum value and corresponding weight
    let max_value = *dp.iter().max().unwrap();
    let total_weight = weight_used.iter().position(|&w| w > 0).unwrap_or(0);

    (max_value, total_weight)
}
