use std::collections::VecDeque;
use std::io::{self, Read};

const QMAX: usize = 102;

#[derive(Clone)]
struct Device {
    // use == true  => active
    // use == false => charging
    use_active: bool,
    // remaining time in current state
    r: i32,
    // index into cycle arrays
    idx: usize,
    // number of pairs
    n: usize,
    // active durations
    u: Vec<i32>,
    // charging durations
    c: Vec<i32>,
    // last time unit when this device was processed in the "active decrement" loop
    // (used to mimic the C code's tp->t logic)
    last_t: i32,
}

fn main() {
    // Read all input as integers.
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
        let lim: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n <= 0 {
            break;
        }
        let n_usize = n as usize;

        let mut devices: Vec<Device> = Vec::with_capacity(n_usize);

        for _ in 0..n_usize {
            let mut u: Vec<i32> = Vec::new();
            let mut c: Vec<i32> = Vec::new();

            // Read pairs until sentinel u <= 0.
            loop {
                let uu: i32 = it.next().unwrap().parse().unwrap();
                if uu <= 0 {
                    break;
                }
                let cc: i32 = it.next().unwrap().parse().unwrap();
                u.push(uu);
                c.push(cc);
            }

            // Per spec, at least one pair exists.
            let r0 = u[0];

            devices.push(Device {
                use_active: true,
                r: r0,
                idx: 0,
                n: u.len(),
                u,
                c,
                last_t: 0,
            });
        }

        let mut ans: i64 = 0;
        let mut que: VecDeque<usize> = VecDeque::new(); // waiting order for charging service

        // Simulate time units 1..=lim
        for t in 1..=lim {
            // First pass: decrement active devices; those that finish become charging and are enqueued.
            for (i, dev) in devices.iter_mut().enumerate() {
                if dev.use_active {
                    dev.last_t = t;
                    dev.r -= 1;
                    if dev.r == 0 {
                        dev.use_active = false;
                        dev.r = dev.c[dev.idx];
                        // Fixed-capacity ring in C; here we mimic capacity limit.
                        // If exceeded, behavior is unspecified by spec; we just push (may diverge).
                        if que.len() < QMAX {
                            que.push_back(i);
                        } else {
                            que.push_back(i);
                        }
                    }
                }
            }

            if que.is_empty() {
                continue;
            }

            // Second pass: for charging devices that were NOT processed in first pass this time unit
            // (i.e., those already charging at start of this time unit), count unserviced ones,
            // and decrement the serviced one by 1.
            let serviced_idx = *que.front().unwrap();
            for (i, dev) in devices.iter_mut().enumerate() {
                if dev.last_t != t && !dev.use_active {
                    if i == serviced_idx {
                        dev.r -= 1;
                    } else {
                        ans += 1;
                    }
                }
            }

            // After service decrement, if the serviced device finished charging, transition it to active
            // and pop it from the queue.
            if let Some(&front) = que.front() {
                if devices[front].r == 0 {
                    let dev = &mut devices[front];
                    dev.use_active = true;
                    dev.idx += 1;
                    if dev.idx == dev.n {
                        dev.idx = 0;
                    }
                    dev.r = dev.u[dev.idx];
                    que.pop_front();
                }
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
