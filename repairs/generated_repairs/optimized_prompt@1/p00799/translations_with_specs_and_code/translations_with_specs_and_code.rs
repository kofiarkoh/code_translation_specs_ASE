// AOJ 1219: Pump up Batteries
// Translated from C to Rust

use std::io::{self, BufRead};

const QMAX: usize = 102;

struct Battery {
    use_battery: bool,
    time: usize,
    recharge_index: usize,
    remaining_time: usize,
    usage_times: Vec<usize>,
    recharge_times: Vec<usize>,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let lim: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 || lim == 0 {
            break; // Terminate if non-positive integers are encountered
        }

        let mut batteries: Vec<Battery> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut usage_times = Vec::new();
            let mut recharge_times = Vec::new();

            // Read usage and recharge times
            while let Some(usage_line) = lines.next() {
                let usage_line = usage_line.unwrap();
                let mut usage_iter = usage_line.split_whitespace();
                let usage: usize = usage_iter.next().unwrap().parse().unwrap();
                if usage == 0 {
                    break; // End of battery input
                }
                let recharge: usize = usage_iter.next().unwrap().parse().unwrap();
                usage_times.push(usage);
                recharge_times.push(recharge);
            }

            if !usage_times.is_empty() && !recharge_times.is_empty() {
                batteries.push(Battery {
                    use_battery: true,
                    time: 0,
                    recharge_index: 0,
                    remaining_time: usage_times[0],
                    usage_times,
                    recharge_times,
                });
            }
        }

        let mut ans = 0;
        let mut queue: Vec<usize> = Vec::new();
        let mut top = 0;
        let mut end = 0;

        for t in 1..=lim {
            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.use_battery {
                    battery.time = t;
                    if battery.remaining_time > 0 {
                        battery.remaining_time -= 1;
                    }
                    if battery.remaining_time == 0 {
                        battery.use_battery = false;
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
                if battery.time != t && !battery.use_battery {
                    if i == queue[top] {
                        battery.remaining_time -= 1;
                    } else {
                        ans += 1;
                    }
                }
            }

            if top < end {
                let battery_index = queue[top];
                let battery = &mut batteries[battery_index];
                if battery.remaining_time == 0 {
                    battery.use_battery = true;
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
