// AOJ 1219: Pump up Batteries
// Translated from C to Rust

use std::io::{self, BufRead};

const QMAX: usize = 102;

#[derive(Debug)]
struct Battery {
    use_battery: bool,
    t: usize,
    r: usize,
    i: usize,
    n: usize,
    u: Vec<usize>,
    c: Vec<usize>,
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
            break; // Exit if n is zero or negative
        }

        let mut batteries: Vec<Battery> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut u = Vec::new();
            let mut c = Vec::new();
            let mut i = 0;

            while let Some(line) = lines.next() {
                let line = line.unwrap();
                let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let usage = nums.next().unwrap();
                if usage <= 0 {
                    break; // Exit if usage is non-positive
                }
                let charge = nums.next().unwrap();
                u.push(usage);
                c.push(charge);
                i += 1;
            }

            batteries.push(Battery {
                use_battery: true,
                t: 0,
                r: u[0],
                i: 0,
                n: i,
                u,
                c,
            });
        }

        let mut ans = 0;
        let mut queue: Vec<usize> = Vec::with_capacity(QMAX);
        let mut top = 0;
        let mut end = 0;

        for t in 1..=lim {
            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.use_battery {
                    battery.t = t;
                    if battery.r > 0 {
                        battery.r -= 1;
                    }
                    if battery.r == 0 {
                        battery.use_battery = false;
                        battery.r = battery.c[battery.i];
                        queue.push(i);
                        end += 1;
                        if end == QMAX {
                            end = 0; // Wrap around
                        }
                    }
                }
            }

            if end == top {
                continue; // Queue is empty
            }

            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.t != t && !battery.use_battery {
                    if i == queue[top] {
                        battery.r -= 1;
                    } else {
                        ans += 1;
                    }
                }
            }

            if top != end {
                let battery_index = queue[top];
                let battery = &mut batteries[battery_index];
                if battery.r == 0 {
                    battery.use_battery = true;
                    battery.i += 1;
                    if battery.i == battery.n {
                        battery.i = 0;
                    }
                    battery.r = battery.u[battery.i];
                    top += 1;
                    if top == QMAX {
                        top = 0; // Wrap around
                    }
                }
            }
        }
        println!("{}", ans);
    }
}
