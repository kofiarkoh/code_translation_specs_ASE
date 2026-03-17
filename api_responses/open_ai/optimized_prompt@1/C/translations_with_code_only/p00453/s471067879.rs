use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    while input.read_line(&mut buffer).unwrap() > 0 {
        let mut parts = buffer.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        if n == 0 && m == 0 {
            break;
        }

        let mut c = [[[2100000000; 11]; 150]; 80];
        c[1][1] = [0; 11];

        let mut ic = vec![0; n];
        let mut is = vec![vec![vec![0; 10]; n]; 2];

        for i in 0..n {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            ic[i] = buffer.trim().parse().unwrap();
            for j in 0..ic[i] {
                buffer.clear();
                input.read_line(&mut buffer).unwrap();
                let coords: Vec<i32> = buffer.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                is[0][i][j] = coords[0];
                is[1][i][j] = coords[1];
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
        buffer.clear();
    }
}
