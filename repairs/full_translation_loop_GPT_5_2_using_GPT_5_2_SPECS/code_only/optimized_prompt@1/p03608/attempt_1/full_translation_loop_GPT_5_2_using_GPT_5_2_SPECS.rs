use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn dfs(
    c: i32,
    las: i32,
    dist: i32,
    r: &[i32; 9],
    used: &mut [bool; 9],
    d: &[[i32; 201]; 201],
    r_count: i32,
    res: &mut i32,
) {
    if c == r_count + 1 {
        if *res > dist {
            *res = dist;
        }
        return;
    }

    for i in 1..=r_count as usize {
        if !used[i] {
            used[i] = true;
            if las == -1 {
                dfs(c + 1, i as i32, 0, r, used, d, r_count, res);
            } else {
                let from = r[las as usize] as usize;
                let to = r[i] as usize;
                // C uses signed int arithmetic; here values are bounded by INF and typical constraints.
                dfs(c + 1, i as i32, dist + d[from][to], r, used, d, r_count, res);
            }
            used[i] = false;
        }
    }
}

fn main() {
    // Read all input (scanf-like whitespace parsing)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let r_count: i32 = it.next().unwrap().parse().unwrap();

    let mut d = [[0i32; 201]; 201];
    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    let mut r = [0i32; 9];
    for i in 1..=r_count as usize {
        r[i] = it.next().unwrap().parse().unwrap();
    }

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let c: i32 = it.next().unwrap().parse().unwrap();
        if d[a][b] > c {
            d[a][b] = c;
            d[b][a] = c;
        }
    }

    // Floyd-Warshall
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                let via = d[i][k] + d[k][j];
                if d[i][j] > via {
                    d[i][j] = via;
                }
            }
        }
    }

    let mut res: i32 = INF;
    let mut used = [false; 9];
    dfs(1, -1, 0, &r, &mut used, &d, r_count, &mut res);

    print!("{res}\n");
}
