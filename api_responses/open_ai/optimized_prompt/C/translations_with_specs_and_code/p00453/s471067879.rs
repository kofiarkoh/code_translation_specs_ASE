use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut ic = vec![0; n];
        let mut is = vec![vec![vec![0; 10]; n]; 2];
        let mut c = vec![vec![vec![2100000000; 11]; 150]; 80];

        c[1][1][0] = 0; // Initialize the starting point

        for i in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            ic[i] = iter.next().unwrap().parse().unwrap();
            for j in 0..ic[i] {
                is[0][i][j] = iter.next().unwrap().parse().unwrap();
                is[1][i][j] = iter.next().unwrap().parse().unwrap();
            }
        }

        for i in 0..=m {
            for j in 0..n - 1 {
                for k in 0..ic[j] {
                    for l in 0..ic[j + 1] {
                        let d = (is[1][j][k] + is[1][j + 1][l]) * (is[0][j][k] - is[0][j + 1][l]).abs();
                        if c[i][j + 1][l] > c[i][j][k] + d {
                            c[i][j + 1][l] = c[i][j][k] + d;
                        }
                    }
                    if j + 2 == n {
                        continue;
                    }
                    for l in 0..ic[j + 2] {
                        let d = (is[1][j][k] + is[1][j + 2][l]) * (is[0][j][k] - is[0][j + 2][l]).abs();
                        if c[i + 1][j + 2][l] > c[i][j][k] + d {
                            c[i + 1][j + 2][l] = c[i][j][k] + d;
                        }
                    }
                }
            }
        }

        let mut min = 2100000210;
        for i in 0..=m {
            for j in 0..ic[n - 1] {
                if min > c[i][n - 1][j] {
                    min = c[i][n - 1][j];
                }
            }
            if m == i {
                continue;
            }
            for j in 0..ic[n - 2] {
                if min > c[i][n - 2][j] {
                    min = c[i][n - 2][j];
                }
            }
        }
        println!("{}", min);
    }
}
