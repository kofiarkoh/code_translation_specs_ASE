use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn dfs(c: i32, las: i32, dist: i32, r: &[usize], used: &mut [bool], d: &Vec<Vec<i32>>, res: &mut i32, r_count: i32) {
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
                dfs(c + 1, i as i32, 0, r, used, d, res, r_count);
            } else {
                let las_u = las as usize;
                // Match C's 32-bit signed overflow behavior as closely as possible.
                let add = d[r[las_u]][r[i]];
                let new_dist = dist.wrapping_add(add);
                dfs(c + 1, i as i32, new_dist, r, used, d, res, r_count);
            }
            used[i] = false;
        }
    }
}

fn main() {
    // Read all input (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let r_count: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays sized like the C code.
    let mut d = vec![vec![0i32; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            if i != j {
                d[i][j] = INF;
            }
        }
    }

    let mut r = vec![0usize; r_count + 1];
    for i in 1..=r_count {
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

    // Floyd-Warshall; use wrapping_add to emulate potential signed overflow in C.
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                let via = d[i][k].wrapping_add(d[k][j]);
                if d[i][j] > via {
                    d[i][j] = via;
                }
            }
        }
    }

    let mut res: i32 = INF;
    let mut used = vec![false; r_count + 1];
    dfs(1, -1, 0, &r, &mut used, &d, &mut res, r_count as i32);

    print!("{res}\n");
}
