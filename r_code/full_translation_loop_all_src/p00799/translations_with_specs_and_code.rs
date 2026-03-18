use std::io::{self, Read};

const QMAX: usize = 102;

#[derive(Clone)]
struct Battery {
    use_flag: bool, // corresponds to C's char use (1/0)
    t: i32,         // last time this battery was processed as "in use" during current tick
    r: i32,         // remaining time in current phase (use or charge)
    i: usize,       // index into u/c arrays
    n: usize,       // number of (u,c) pairs
    u: Vec<i32>,    // usage times
    c: Vec<i32>,    // charge times
}

fn main() {
    // Read all input; behavior is undefined on malformed input per spec/C.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let lim: i32 = it.next().unwrap().parse().unwrap();

        // Terminates gracefully when a non-positive integer is encountered for number of batteries.
        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut tbl: Vec<Battery> = Vec::with_capacity(n_usize);

        for _ in 0..n_usize {
            let mut u_vec: Vec<i32> = Vec::new();
            let mut c_vec: Vec<i32> = Vec::new();

            // Read pairs (u, c) until u <= 0.
            loop {
                let u: i32 = it.next().unwrap().parse().unwrap();
                if u <= 0 {
                    break;
                }
                let c: i32 = it.next().unwrap().parse().unwrap();
                u_vec.push(u);
                c_vec.push(c);
            }

            // Preconditions say at least one usage/recharge pair exists.
            let r0 = u_vec[0];

            tbl.push(Battery {
                use_flag: true,
                t: 0,
                r: r0,
                i: 0,
                n: u_vec.len(),
                u: u_vec,
                c: c_vec,
            });
        }

        // Circular queue of battery indices.
        let mut que = vec![0usize; QMAX + 2]; // a bit extra, like C's 104
        let mut top: usize = 0;
        let mut end: usize = 0;

        let mut ans: i32 = 0;

        for t in 1..=lim {
            // Step 1: advance all batteries that are currently in use.
            for (idx, b) in tbl.iter_mut().enumerate() {
                if b.use_flag {
                    b.t = t;
                    b.r -= 1;
                    if b.r == 0 {
                        b.use_flag = false;
                        b.r = b.c[b.i];
                        que[end] = idx;
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

            // Step 2: for batteries not processed as "in use" this tick and currently not in use,
            // decrement the one at the head of the queue; count others as "usage without recharge".
            for (idx, b) in tbl.iter_mut().enumerate() {
                if b.t != t && !b.use_flag {
                    if idx == que[top] {
                        b.r -= 1;
                    } else {
                        ans += 1;
                    }
                }
            }

            // Step 3: if the head-of-queue battery finished charging, make it active again and pop.
            if top != end {
                let head = que[top];
                if tbl[head].r == 0 {
                    let b = &mut tbl[head];
                    b.use_flag = true;
                    b.i += 1;
                    if b.i == b.n {
                        b.i = 0;
                    }
                    b.r = b.u[b.i];

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
