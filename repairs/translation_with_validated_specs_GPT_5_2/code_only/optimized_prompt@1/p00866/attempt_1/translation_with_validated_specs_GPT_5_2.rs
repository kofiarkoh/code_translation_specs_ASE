use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct T {
    t: i32,
    id: usize,
}

fn check(id: usize, t: &[i32; 3], a: &[[i32; 60]; 12], c: &[[usize; 3]; 6], tbl: &mut Vec<T>) {
    for j in 0..60i32 {
        for i in 0..6usize {
            let h = (t[c[i][0]] + j).rem_euclid(60);
            let hh = (h / 5) as usize;
            let m = (t[c[i][1]] + j).rem_euclid(60) as usize;
            let s = (t[c[i][2]] + j).rem_euclid(60);

            if a[hh][m] == h {
                let tt = (((hh as i32) * 60 + (m as i32)) * 60) + s;
                tbl.push(T { t: tt, id });
            }
        }
    }
}

fn main() {
    // Read all input; emulate scanf-style tokenization.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Precompute a[12][60] = (60*i + j)/12
    let mut a = [[0i32; 60]; 12];
    for i in 0..12 {
        for j in 0..60 {
            a[i][j] = ((60 * i + j) / 12) as i32;
        }
    }

    let c: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut tbl: Vec<T> = Vec::new();
        for i in 0..n {
            let t0: i32 = it.next().unwrap().parse().unwrap();
            let t1: i32 = it.next().unwrap().parse().unwrap();
            let t2: i32 = it.next().unwrap().parse().unwrap();
            let t = [t0, t1, t2];
            check(i, &t, &a, &c, &mut tbl);
        }

        // qsort with cmp: return a->t - b->t (can overflow in C, but t is within [0, 43199])
        tbl.sort_by(|x, y| x.t.cmp(&y.t));

        let mut d: i32 = 0x7fffffff;
        let mut min_t: i32 = 0;
        let mut max_t: i32 = 0;

        for i in 0..tbl.len() {
            // C uses char f[11]; assumes n <= 11. Preserve behavior as closely as possible:
            // allocate max(n, 11) so typical cases match; if n > 11, this avoids OOB (C would be UB).
            let mut f = vec![0u8; std::cmp::max(11, n)];
            let mut k: usize = 0;

            for j in i..tbl.len() {
                let id = tbl[j].id;
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
