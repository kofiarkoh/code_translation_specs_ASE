// AOJ 1219: Pump up Batteries
// 2017.10.23 bal4u@uu

use std::io::{self, BufRead};

const QMAX: usize = 102;

#[derive(Default)]
struct T {
    use_flag: u8, // use is a char in C, using u8 for Rust
    t: usize,
    r: usize,
    i: usize,
    n: usize,
    u: [usize; 51],
    c: [usize; 51],
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let lim = nums.next().unwrap();

        if n == 0 {
            break;
        }

        let mut tbl: Vec<T> = vec![T::default(); n];
        for tp in &mut tbl {
            tp.use_flag = 1;
            tp.i = 0;
            let mut i = 0;
            while let Some(line) = lines.next() {
                let line = line.unwrap();
                let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let u = nums.next().unwrap();
                if u <= 0 {
                    break;
                }
                let c = nums.next().unwrap();
                tp.u[i] = u;
                tp.c[i] = c;
                i += 1;
            }
            tp.n = i;
            tp.r = tp.u[0];
        }

        let mut ans = 0;
        let mut top = 0;
        let mut end = 0;
        let mut que = vec![0; QMAX];

        for t in 1..=lim {
            for (i, tp) in tbl.iter_mut().enumerate() {
                if tp.use_flag != 0 {
                    tp.t = t;
                    if tp.r > 0 {
                        tp.r -= 1;
                    }
                    if tp.r == 0 {
                        tp.use_flag = 0;
                        tp.r = tp.c[tp.i];
                        que[end] = i;
                        end = (end + 1) % QMAX;
                    }
                }
            }

            if end == top {
                continue;
            }

            for (i, tp) in tbl.iter_mut().enumerate() {
                if tp.t != t && tp.use_flag == 0 {
                    if i == que[top] {
                        tp.r = tp.r.saturating_sub(1); // Prevent underflow
                    } else {
                        ans += 1;
                    }
                }
            }

            if top != end {
                let tp = &mut tbl[que[top]];
                if tp.r == 0 {
                    tp.use_flag = 1;
                    tp.i = (tp.i + 1) % tp.n;
                    tp.r = tp.u[tp.i];
                    top = (top + 1) % QMAX;
                }
            }
        }
        println!("{}", ans);
    }
}
