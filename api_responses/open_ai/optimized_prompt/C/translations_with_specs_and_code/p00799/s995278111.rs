// AOJ 1219: Pump up Batteries
// Translated from C to Rust

use std::io::{self, BufRead};

const QMAX: usize = 102;

struct Battery {
    active: bool,
    remaining_time: usize,
    recharge_index: usize,
    usage_times: Vec<usize>,
    recharge_times: Vec<usize>,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let lim: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 || lim == 0 {
            break; // Terminate if non-positive input
        }

        let mut batteries: Vec<Battery> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut usage_times = Vec::new();
            let mut recharge_times = Vec::new();
            let mut u: usize;
            let mut c: usize;

            while let Some(usage) = iter.next() {
                u = usage.parse().unwrap();
                if u <= 0 {
                    break; // Ignore non-positive usage
                }
                c = iter.next().unwrap().parse().unwrap();
                usage_times.push(u);
                recharge_times.push(c);
            }

            batteries.push(Battery {
                active: true,
                remaining_time: usage_times[0],
                recharge_index: 0,
                usage_times,
                recharge_times,
            });
        }

        let mut ans = 0;
        let mut queue: Vec<usize> = Vec::new();
        let mut top = 0;
        let mut end = 0;

        for t in 1..=lim {
            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.active {
                    if battery.remaining_time > 0 {
                        battery.remaining_time -= 1;
                    }
                    if battery.remaining_time == 0 {
                        battery.active = false;
                        battery.remaining_time = battery.recharge_times[battery.recharge_index];
                        queue.push(i);
                        end += 1;
                    }
                }
            }

            if end == top {
                continue; // No batteries to process
            }

            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.active {
                    continue; // Skip active batteries
                }
                if i == queue[top] {
                    battery.remaining_time -= 1;
                } else {
                    ans += 1;
                }
            }

            if top < end {
                let idx = queue[top];
                let battery = &mut batteries[idx];
                if battery.remaining_time == 0 {
                    battery.active = true;
                    battery.recharge_index += 1;
                    if battery.recharge_index == battery.usage_times.len() {
                        battery.recharge_index = 0;
                    }
                    battery.remaining_time = battery.usage_times[battery.recharge_index];
                    top += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
