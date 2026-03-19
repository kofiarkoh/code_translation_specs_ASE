use std::io::{self, Read};

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().unwrap() as usize,
        None => return,
    };
    let m: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut a: Vec<usize> = vec![0; m];
    let mut b: Vec<usize> = vec![0; m];
    let mut c: Vec<i32> = vec![0; m];

    for i in 0..m {
        let ai = it.next().unwrap().parse::<i32>().unwrap() - 1;
        let bi = it.next().unwrap().parse::<i32>().unwrap() - 1;
        let ci = it.next().unwrap().parse::<i32>().unwrap();
        a[i] = ai as usize;
        b[i] = bi as usize;
        c[i] = ci;
    }

    // dist initialized to 0 on diagonal, 1_000_000_000 otherwise.
    let inf: i32 = 1_000_000_000;
    let mut dist: Vec<Vec<i32>> = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Undirected edges with min weight if multiple edges exist.
    for i in 0..m {
        let ai = a[i];
        let bi = b[i];
        let ci = c[i];
        dist[ai][bi] = min_i32(dist[ai][bi], ci);
        dist[bi][ai] = min_i32(dist[bi][ai], ci);
    }

    // Floyd–Warshall.
    // In C, `dist[i][k] + dist[k][j]` is signed int addition; overflow would be UB.
    // Here we use wrapping_add to emulate typical two's-complement wraparound behavior.
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let via = dist[i][k].wrapping_add(dist[k][j]);
                dist[i][j] = min_i32(dist[i][j], via);
            }
        }
    }

    let mut ans: i32 = m as i32;

    for i in 0..m {
        let mut f: i32 = 1;
        for j in 0..n {
            let lhs = dist[j][a[i]].wrapping_add(c[i]);
            if lhs == dist[j][b[i]] {
                f = 0;
            }
        }
        if f == 0 {
            ans -= 1;
        }
    }

    print!("{}\n", ans);
}
