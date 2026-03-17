use std::io::{self, BufRead};

const MAX_N: usize = 201;
const INF: i32 = 1_000_000_000;

fn dfs(c: usize, las: isize, dist: i32, r: &[usize], d: &[[i32; MAX_N]; MAX_N], used: &mut [bool; 9], res: &mut i32) {
    if c == r.len() {
        if *res > dist {
            *res = dist;
        }
        return;
    }

    for i in 0..r.len() {
        if !used[i] {
            used[i] = true;
            if las == -1 {
                dfs(c + 1, i as isize, 0, r, d, used, res);
            } else {
                dfs(c + 1, i as isize, dist + d[r[las as usize]][r[i]], r, d, used, res);
            }
            used[i] = false;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let m: usize = params.next().unwrap().parse().unwrap();
    let r_count: usize = params.next().unwrap().parse().unwrap();

    let mut d = [[INF; MAX_N]; MAX_N];
    let mut r = vec![0; r_count];
    let mut used = vec![false; r_count];
    let mut res = INF;

    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    for i in 0..r_count {
        let line = lines.next().unwrap().unwrap();
        r[i] = line.trim().parse::<usize>().unwrap(); // Ensure parsing to usize
    }

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edges = line.split_whitespace();
        let a: usize = edges.next().unwrap().parse().unwrap();
        let b: usize = edges.next().unwrap().parse().unwrap();
        let c: i32 = edges.next().unwrap().parse().unwrap();
        if d[a][b] > c {
            d[a][b] = c;
            d[b][a] = c;
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    dfs(0, -1, 0, &r, &d, &mut used, &mut res);

    println!("{}", res);
}
