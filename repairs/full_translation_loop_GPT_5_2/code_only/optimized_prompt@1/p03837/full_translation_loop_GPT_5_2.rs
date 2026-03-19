use std::io::{self, Read};

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn main() {
    // Read entire stdin (matches scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = vec![0; m];
    let mut b: Vec<usize> = vec![0; m];
    let mut c: Vec<i32> = vec![0; m];

    for i in 0..m {
        let ai: i32 = it.next().unwrap().parse().unwrap();
        let bi: i32 = it.next().unwrap().parse().unwrap();
        let ci: i32 = it.next().unwrap().parse().unwrap();
        // C code decrements after reading (1-based to 0-based).
        a[i] = (ai - 1) as usize;
        b[i] = (bi - 1) as usize;
        c[i] = ci;
    }

    // dist initialized to 0 on diagonal, 1_000_000_000 otherwise.
    let inf: i32 = 1_000_000_000;
    let mut dist: Vec<Vec<i32>> = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Multiple edges: keep minimum weight.
    for i in 0..m {
        let ai = a[i];
        let bi = b[i];
        let ci = c[i];
        dist[ai][bi] = min_i32(dist[ai][bi], ci);
        dist[bi][ai] = min_i32(dist[bi][ai], ci);
    }

    // Floyd-Warshall.
    // Note: In C, signed overflow is undefined; typical contest constraints avoid it.
    // We use wrapping_add to emulate common two's-complement behavior if overflow occurs.
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
            if dist[j][a[i]].wrapping_add(c[i]) == dist[j][b[i]] {
                f = 0;
            }
        }
        if f == 0 {
            ans -= 1;
        }
    }

    print!("{}\n", ans);
}
