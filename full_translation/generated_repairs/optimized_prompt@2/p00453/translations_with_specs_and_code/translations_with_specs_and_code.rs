use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if n == 0 && m == 0 {
            break;
        }

        let mut ic = vec![0; n];
        let mut is = vec![vec![vec![(0, 0); 150]; 2]; n]; // (coordinate, weight)
        let mut c = vec![vec![vec![i32::MAX; 11]; 150]; 80];

        // Initialize the cost array
        for i in 0..80 {
            for j in 1..150 {
                for k in 0..11 {
                    c[i][j][k] = i32::MAX;
                }
            }
        }
        c[1][1][0] = 0; // Starting condition

        // Read point sets
        for i in 0..n {
            let line = reader.lines().next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            ic[i] = iter.next().unwrap().parse().unwrap();
            for j in 0..ic[i] {
                let x: i32 = iter.next().unwrap().parse().unwrap();
                let w: i32 = iter.next().unwrap().parse().unwrap();
                is[i][0][j] = (x, w);
            }
        }

        // Compute minimum costs
        for i in 0..=m {
            for j in 0..n - 1 {
                for k in 0..ic[j] {
                    for l in 0..ic[j + 1] {
                        let d = (is[j][1][k].1 + is[j + 1][1][l].1) * (is[j][0][k].0 - is[j + 1][0][l].0).abs();
                        if c[i][j + 1][l] > c[i][j][k] + d {
                            c[i][j + 1][l] = c[i][j][k] + d;
                        }
                    }
                    if j + 2 == n {
                        continue;
                    }
                    for l in 0..ic[j + 2] {
                        let d = (is[j][1][k].1 + is[j + 2][1][l].1) * (is[j][0][k].0 - is[j + 2][0][l].0).abs();
                        if c[i + 1][j + 2][l] > c[i][j][k] + d {
                            c[i + 1][j + 2][l] = c[i][j][k] + d;
                        }
                    }
                }
            }
        }

        // Find the minimum cost
        let mut min = i32::MAX;
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
