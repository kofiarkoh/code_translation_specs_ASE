use std::io::{self, BufRead};

const INF: i32 = 1_000_000_000;
const MAX_N: usize = 201;
const MAX_R: usize = 9;

fn dfs(c: usize, las: i32, dist: i32, r: &[i32], d: &[[i32; MAX_N]; MAX_N], used: &mut [bool; MAX_R + 1], res: &mut i32, R: usize) {
    if c == R + 1 {
        if *res > dist {
            *res = dist;
        }
        return;
    }

    for i in 1..=R {
        if !used[i] {
            used[i] = true;
            if las == -1 {
                dfs(c + 1, i as i32, 0, r, d, used, res, R);
            } else {
                dfs(c + 1, i as i32, dist + d[r[las as usize] as usize][r[i as usize] as usize], r, d, used, res, R);
            }
            used[i] = false;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let N: usize = parts.next().unwrap().parse().unwrap();
    let M: usize = parts.next().unwrap().parse().unwrap();
    let R: usize = parts.next().unwrap().parse().unwrap();

    let mut d = [[INF; MAX_N]; MAX_N];
    let mut r = [0; MAX_R + 1];
    let mut used = [false; MAX_R + 1];
    let mut res = INF;

    for i in 1..=N {
        for j in 1..=N {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    for i in 1..=R {
        let line = lines.next().unwrap().unwrap();
        r[i] = line.trim().parse().expect("Failed to parse input for r[i]");
    }

    for _ in 1..=M {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let A: usize = parts.next().unwrap().parse().expect("Failed to parse input for A");
        let B: usize = parts.next().unwrap().parse().expect("Failed to parse input for B");
        let C: i32 = parts.next().unwrap().parse().expect("Failed to parse input for C");
        if d[A][B] > C {
            d[A][B] = C;
            d[B][A] = C;
        }
    }

    for k in 1..=N {
        for i in 1..=N {
            for j in 1..=N {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    dfs(1, -1, 0, &r, &d, &mut used, &mut res, R);

    println!("{}", res);
}
