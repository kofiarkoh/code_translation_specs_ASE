use std::io::{self, Read};

const QMAX: usize = 102;

#[derive(Clone)]
struct T {
    use_flag: bool, // corresponds to char use (0/1)
    t: i32,
    r: i32, // remain time
    i: usize,
    n: usize,
    u: [i32; 51],
    c: [i32; 51],
}

fn main() {
    // Read entire stdin, parse as integers like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let lim: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut tbl: Vec<T> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let mut tp = T {
                use_flag: true,
                t: 0,
                r: 0,
                i: 0,
                n: 0,
                u: [0; 51],
                c: [0; 51],
            };

            let mut idx: usize = 0;
            loop {
                // C: for (i = 0; scanf("%d", &u) && u > 0; i++) scanf("%d",&c) ...
                let u_val: i32 = match it.next() {
                    Some(x) => x.parse().unwrap(),
                    None => 0, // emulate "scanf fails" -> loop ends
                };
                if u_val <= 0 {
                    break;
                }
                let c_val: i32 = match it.next() {
                    Some(x) => x.parse().unwrap(),
                    None => 0, // if missing, behave as if read 0
                };
                if idx < 51 {
                    tp.u[idx] = u_val;
                    tp.c[idx] = c_val;
                }
                idx += 1;
            }
            // In the original problem constraints, idx should be <= 51.
            // We preserve the C behavior for valid inputs; for invalid oversized inputs,
            // we keep only the first 51 entries (avoids out-of-bounds in Rust).
            tp.n = idx.min(51);
            tp.r = tp.u[0];
            tbl.push(tp);
        }

        let mut ans: i32 = 0;
        let mut que = [0usize; 104];
        let mut top: usize = 0;
        let mut end: usize = 0;

        for t in 1..=lim {
            // First phase: decrement r for those in use; enqueue those that reach 0.
            for i in 0..n_usize {
                if tbl[i].use_flag {
                    tbl[i].t = t;
                    tbl[i].r -= 1;
                    if tbl[i].r == 0 {
                        tbl[i].use_flag = false;
                        let ci = tbl[i].i;
                        tbl[i].r = tbl[i].c[ci];
                        que[end] = i;
                        end += 1;
                        if end == QMAX {
                            end = 0;
                        }
                    }
                }
            }

            if end == top {
                continue;
            }

            // Second phase: for those not touched this time (t != current) and not in use,
            // decrement r for the one at queue[top], else count ans++.
            for i in 0..n_usize {
                if tbl[i].t != t && !tbl[i].use_flag {
                    if i == que[top] {
                        tbl[i].r -= 1;
                    } else {
                        ans += 1;
                    }
                }
            }

            // Third phase: if the queued one finished charging, switch it to use and pop.
            if top != end {
                let qi = que[top];
                if tbl[qi].r == 0 {
                    tbl[qi].use_flag = true;
                    tbl[qi].i += 1;
                    if tbl[qi].i == tbl[qi].n {
                        tbl[qi].i = 0;
                    }
                    let ui = tbl[qi].i;
                    tbl[qi].r = tbl[qi].u[ui];
                    top += 1;
                    if top == QMAX {
                        top = 0;
                    }
                }
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
