use std::io::{self, BufRead};

const MAX_ROWS: usize = 99;
const MAX_COLS: usize = 50;

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn judge(data: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, i: usize, j: usize, sum: i32, n: usize) -> i32 {
    let mut sum = sum + data[i][j];

    if i == n || sum < dp[i][j] {
        return sum;
    } else {
        dp[i][j] = sum;

        if i < n / 2 {
            return max(judge(data, dp, i + 1, j, dp[i][j], n), judge(data, dp, i + 1, j + 1, dp[i][j], n));
        } else {
            return max(judge(data, dp, i + 1, j.wrapping_sub(1), dp[i][j], n), judge(data, dp, i + 1, j, dp[i][j], n));
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut data: Vec<Vec<i32>> = vec![vec![0; MAX_COLS]; MAX_ROWS];
    let mut dp: Vec<Vec<i32>> = vec![vec![0; MAX_COLS]; MAX_ROWS];
    let mut n = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if n < MAX_ROWS {
            for (j, &num) in numbers.iter().enumerate() {
                if j < MAX_COLS {
                    data[n][j + 1] = num; // Start from index 1 to match C code
                }
            }
            n += 1;
        }
    }

    let ans = judge(&data, &mut dp, 0, 1, 0, n);
    println!("{}", ans);
}
