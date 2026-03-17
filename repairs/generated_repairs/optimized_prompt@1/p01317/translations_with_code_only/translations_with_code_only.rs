use std::io::{self, BufRead};
use std::cmp::min;

const MN: usize = 205;
const INF: i32 = 100_000_000;

fn init<T: Default>(a: &mut [[T; MN]; MN]) {
    for row in a.iter_mut() {
        for elem in row.iter_mut() {
            *elem = Default::default(); // Initialize with default value
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut de = vec![0; 1005];
        let mut s = [[0x0f; MN]; MN];
        let mut l = [[0x0f; MN]; MN];
        let mut dp = [[INF; MN]; 1005]; // Changed from Vec<Vec<i32>> to [[i32; MN]; 1005]

        init(&mut s);
        init(&mut l);
        init(&mut dp);

        for i in 0..n {
            l[i][i] = 0;
            s[i][i] = 0;
        }

        for _ in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let x: usize = iter.next().unwrap().parse().unwrap() - 1;
            let y: usize = iter.next().unwrap().parse().unwrap() - 1;
            let s_value: i32 = iter.next().unwrap().parse().unwrap();
            let t: char = iter.next().unwrap().chars().next().unwrap();

            if t == 'L' {
                l[x][y] = s_value;
                l[y][x] = s_value;
            } else {
                s[x][y] = s_value;
                s[y][x] = s_value;
            }
        }

        let r: usize = lines.next().unwrap().unwrap().parse().unwrap();
        for i in 0..r {
            let line = lines.next().unwrap().unwrap();
            de[i] = line.trim().parse().unwrap();
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
