use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    t: i32,
    id: i32,
}

fn check(id: i32, t: &[i32; 3], a: &[[i32; 60]; 12], tbl: &mut Vec<T>) {
    // Permutations of indices (same as C's c[6][3])
    const C: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    for j in 0..60i32 {
        for perm in C.iter() {
            let h = (t[perm[0]] + j).rem_euclid(60);
            let hh = h / 5;
            let m = (t[perm[1]] + j).rem_euclid(60);
            let s = (t[perm[2]] + j).rem_euclid(60);
            if a[hh as usize][m as usize] == h {
                let tt = ((hh * 60) + m) * 60 + s;
                tbl.push(T { t: tt, id });
            }
        }
    }
}

fn main() {
    // Read all input (scanf-like whitespace tokenization)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Precompute a[12][60] = (60*i + j)/12
    let mut a = [[0i32; 60]; 12];
    for i in 0..12i32 {
        for j in 0..60i32 {
            a[i as usize][j as usize] = (60 * i + j) / 12;
        }
    }

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }

        let mut tbl: Vec<T> = Vec::new();
        // Upper bound: each watch can contribute at most 60*6 = 360 entries
        tbl.reserve((n as usize) * 360);

        for id in 0..n {
            let t0: i32 = it.next().unwrap().parse().unwrap();
            let t1: i32 = it.next().unwrap().parse().unwrap();
            let t2: i32 = it.next().unwrap().parse().unwrap();
            let t = [t0, t1, t2];
            check(id, &t, &a, &mut tbl);
        }

        // qsort with cmp(a->t - b->t): use stable numeric ordering (no overflow here in practice)
        tbl.sort_by_key(|e| e.t);

        let mut d: i32 = 0x7fffffff;
        let mut min_t: i32 = 0;
        let mut max_t: i32 = 0;

        // f[11] in C; keep same size to preserve any implicit constraints
        let mut f = [0u8; 11];

        for i in 0..tbl.len() {
            f.fill(0);
            let mut k: i32 = 0;
            for j in i..tbl.len() {
                let id = tbl[j].id as usize;
                if id < f.len() && f[id] != 0 {
                    continue;
                }
                if id < f.len() {
                    f[id] = 1;
                }
                k += 1;
                if k == n {
                    let diff = tbl[j].t - tbl[i].t;
                    if diff < d {
                        d = diff;
                        min_t = tbl[i].t;
                        max_t = tbl[j].t;
                    }
                    break;
                }
            }
        }

        out.push_str(&format!(
            "{:02}:{:02}:{:02} {:02}:{:02}:{:02}\n",
            min_t / 3600,
            (min_t / 60) % 60,
            min_t % 60,
            max_t / 3600,
            (max_t / 60) % 60,
            max_t % 60
        ));
    }

    print!("{out}");
}
