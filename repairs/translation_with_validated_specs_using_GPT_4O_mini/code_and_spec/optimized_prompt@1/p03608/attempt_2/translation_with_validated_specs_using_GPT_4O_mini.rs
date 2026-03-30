use std::io::{self, BufRead};

const MAX_NODES: usize = 201;
const MAX_LOCATIONS: usize = 9;
const INF: i32 = 1_000_000_000;

fn dfs(c: usize, las: isize, dist: i32, r: &[usize], d: &[[i32; MAX_NODES]; MAX_NODES], used: &mut [bool; MAX_LOCATIONS], res: &mut i32) {
    if c == r.len() {
        *res = (*res).min(dist);
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
    let mut params = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = params.next().unwrap();
    let m = params.next().unwrap();
    let r_count = params.next().unwrap();

    let mut d = [[INF; MAX_NODES]; MAX_NODES];
    let mut r = vec![0; MAX_LOCATIONS];

    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    for i in 0..r_count {
        r[i] = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    }

    for _ in 0..m {
        let edge_line = lines.next().unwrap().unwrap();
        let mut edge_params = edge_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = edge_params.next().unwrap();
        let b = edge_params.next().unwrap();
        let c = edge_params.next().unwrap();
        if d[a][b] > c as i32 {
            d[a][b] = c as i32;
            d[b][a] = c as i32;
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

    let mut res = INF;
    let mut used = [false; MAX_LOCATIONS];

    dfs(0, -1, 0, &r, &d, &mut used, &mut res);

    println!("{}", res);
}
