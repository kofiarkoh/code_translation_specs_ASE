use std::io::{self, BufRead};

const MN: usize = 205;
const INF: i32 = 100_000_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut s = vec![vec![INF; MN]; MN];
        let mut l = vec![vec![INF; MN]; MN];
        let mut dp = vec![vec![INF; 205]; 1005];
        let mut de = vec![0; 1005];

        for i in 0..n {
            l[i][i] = 0;
            s[i][i] = 0;
        }

        for _ in 0..m {
            let line = reader.lines().next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();
            let cost: i32 = iter.next().unwrap().parse().unwrap();
            let t: char = iter.next().unwrap().chars().next().unwrap();

            let x = x - 1; // Convert to 0-based index
            let y = y - 1; // Convert to 0-based index

            if t == 'L' {
                l[x][y] = l[y][x] = cost;
            } else {
                s[x][y] = s[y][x] = cost;
            }
        }

        let r: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
        for i in 0..r {
            let line = reader.lines().next().unwrap().unwrap();
            de[i] = line.trim().parse::<usize>().unwrap() - 1; // Convert to 0-based index
        }

        // Floyd-Warshall algorithm
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    s[i][j] = s[i][j].min(s[i][k] + s[k][j]);
                    l[i][j] = l[i][j].min(l[i][k] + l[k][j]);
                }
            }
        }

        dp[0][de[0]] = 0;

        for k in 0..r - 1 {
            for i in 0..n {
                for j in 0..n {
                    dp[k + 1][i] = dp[k + 1][i].min(dp[k][j] + l[de[k]][j] + s[j][i] + l[i][de[k + 1]]);
                    if i == j {
                        dp[k + 1][i] = dp[k + 1][i].min(dp[k][j] + l[de[k]][de[k + 1]]);
                    }
                }
            }
        }

        let mut min_cost = INF;
        for i in 0..n {
            min_cost = min_cost.min(dp[r - 1][i]);
        }

        println!("{}", min_cost);
    }
}
