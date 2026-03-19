use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    loop {
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nums[0];
        let m = nums[1];
        
        if n == 0 && m == 0 {
            break;
        }

        let mut c = [[[2100000000; 11]; 150]; 80];
        c[1][1] = [0; 11];

        let mut ic = vec![0; 150];
        let mut is = vec![vec![vec![0; 10]; 150]; 2];

        for i in 0..n {
            let mut line = String::new();
            input.read_line(&mut line).unwrap();
            let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            ic[i as usize] = nums[0];
            for j in 0..ic[i as usize] {
                let mut line = String::new();
                input.read_line(&mut line).unwrap();
                let coords: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                is[0][i as usize][j as usize] = coords[0];
                is[1][i as usize][j as usize] = coords[1];
            }
        }

        let mut min = 2100000210;

        for i in 0..=m {
            for j in 0..n - 1 {
                for k in 0..ic[j as usize] {
                    for l in 0..ic[(j + 1) as usize] {
                        let d = (is[1][j as usize][k as usize] + is[1][(j + 1) as usize][l as usize])
                            * (is[0][j as usize][k as usize] - is[0][(j + 1) as usize][l as usize]).abs();
                        if c[i as usize][(j + 1) as usize][l as usize] > c[i as usize][j as usize][k as usize] + d {
                            c[i as usize][(j + 1) as usize][l as usize] = c[i as usize][j as usize][k as usize] + d;
                        }
                    }
                    if j + 2 == n {
                        continue;
                    }
                    for l in 0..ic[(j + 2) as usize] {
                        let d = (is[1][j as usize][k as usize] + is[1][(j + 2) as usize][l as usize])
                            * (is[0][j as usize][k as usize] - is[0][(j + 2) as usize][l as usize]).abs();
                        if c[(i + 1) as usize][(j + 2) as usize][l as usize] > c[i as usize][j as usize][k as usize] + d {
                            c[(i + 1) as usize][(j + 2) as usize][l as usize] = c[i as usize][j as usize][k as usize] + d;
                        }
                    }
                }
            }
        }

        for i in 0..=m {
            for j in 0..ic[(n - 1) as usize] {
                if min > c[i as usize][(n - 1) as usize][j as usize] {
                    min = c[i as usize][(n - 1) as usize][j as usize];
                }
            }
            if m == i {
                continue;
            }
            for j in 0..ic[(n - 2) as usize] {
                if min > c[i as usize][(n - 2) as usize][j as usize] {
                    min = c[i as usize][(n - 2) as usize][j as usize];
                }
            }
        }
        println!("{}", min);
    }
}
