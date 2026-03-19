use std::io::{self, BufRead};
use std::cmp::min;

const MN: usize = 205;
const INF: i32 = 100_000_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut s = vec![vec![INF; MN]; MN];
        let mut l = vec![vec![INF; MN]; MN];
        let mut dp = vec![vec![INF; 1005]; 205];
        let mut de = vec![0; 1005];

        for i in 0..n {
            l[i][i] = 0;
            s[i][i] = 0;
        }

        for _ in 0..m {
            let edge_line = lines.next().unwrap().unwrap();
            let mut edge_iter = edge_line.split_whitespace();
            let x: usize = edge_iter.next().unwrap().parse().unwrap();
            let y: usize = edge_iter.next().unwrap().parse().unwrap();
            let cost: i32 = edge_iter.next().unwrap().parse().unwrap();
            let edge_type: char = edge_iter.next().unwrap().chars().next().unwrap();

            let (x, y) = (x - 1, y - 1); // Convert to 0-based index
            if edge_type == 'L' {
                l[x][y] = min(l[x][y], cost);
                l[y][x] = min(l[y][x], cost);
            } else {
                s[x][y] = min(s[x][y], cost);
                s[y][x] = min(s[y][x], cost);
            }
        }

        let r: usize = lines.next().unwrap().unwrap().parse().unwrap();
        for i in 0..r {
            de[i] = lines.next().unwrap().unwrap().parse().unwrap();
            de[i] -= 1; // Convert to 0-based index
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    s[i][j] = min(s[i][j], s[i][k] + s[k][j]);
                    l[i][j] = min(l[i][j], l[i][k] + l[k][j]);
                }
            }
        }

        dp[0][de[0]] = 0;
        for k in 0..r - 1 {
            for i in 0..n {
                for j in 0..n {
                    dp[k + 1][i] = min(dp[k + 1][i], dp[k][j] + l[de[k]][j] + s[j][i] + l[i][de[k + 1]]);
                    if i == j {
                        dp[k + 1][i] = min(dp[k + 1][i], dp[k][j] + l[de[k]][de[k + 1]]);
                    }
                }
            }
        }

        let mut result = INF;
        for i in 0..n {
            result = min(result, dp[r - 1][i]);
        }
        println!("{}", result);
    }
}
