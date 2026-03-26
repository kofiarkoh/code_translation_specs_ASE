use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    while let Some(line) = input.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();

        if n == 0 && m == 0 {
            break;
        }

        let mut ic = vec![0; n];
        let mut is = vec![vec![vec![(0, 0); 10]; n]; 2];
        let mut c = vec![vec![vec![2100000000; 11]; n + 1]; m + 1];

        c[0][0][0] = 0; // Initialize starting point

        for i in 0..n {
            let line = input.next().unwrap().unwrap();
            ic[i] = line.trim().parse::<usize>().unwrap();
            for j in 0..ic[i] {
                let line = input.next().unwrap().unwrap();
                let coords: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
                is[0][i][j] = (coords[0], coords[1]);
            }
        }

        for i in 0..=m {
            for j in 0..n - 1 {
                for k in 0..ic[j] {
                    for l in 0..ic[j + 1] {
                        let d = (is[1][j][k].1 + is[1][j + 1][l].1) * (is[0][j][k].0 - is[0][j + 1][l].0).abs();
                        if c[i][j + 1][l] > c[i][j][k] + d {
                            c[i][j + 1][l] = c[i][j][k] + d;
                        }
                    }
                    if j + 2 < n {
                        for l in 0..ic[j + 2] {
                            let d = (is[1][j][k].1 + is[1][j + 2][l].1) * (is[0][j][k].0 - is[0][j + 2][l].0).abs();
                            if c[i + 1][j + 2][l] > c[i][j][k] + d {
                                c[i + 1][j + 2][l] = c[i][j][k] + d;
                            }
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
