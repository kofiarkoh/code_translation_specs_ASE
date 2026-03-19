use std::io::{self, BufRead};

const QMAX: usize = 102;

struct T {
    use_flag: u8, // use as a boolean (1 or 0)
    t: usize,
    r: usize, // remain time
    i: usize,
    n: usize,
    u: [usize; 51],
    c: [usize; 51],
}

impl Default for T {
    fn default() -> Self {
        T {
            use_flag: 0,
            t: 0,
            r: 0,
            i: 0,
            n: 0,
            u: [0; 51],
            c: [0; 51],
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut tbl: Vec<T> = Vec::new();
    let mut que: [usize; QMAX] = [0; QMAX];
    let mut top = 0;
    let mut end = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let lim: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        tbl.clear();
        for _ in 0..n {
            let mut t = T::default();
            t.use_flag = 1;
            t.i = 0;

            let mut i = 0;
            while let Some(u_str) = iter.next() {
                let u: usize = u_str.parse().unwrap();
                if u <= 0 {
                    break;
                }
                let c: usize = iter.next().unwrap().parse().unwrap();
                t.u[i] = u;
                t.c[i] = c;
                i += 1;
            }
            t.n = i;
            t.r = t.u[0];
            tbl.push(t);
        }

        let mut ans = 0;

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
