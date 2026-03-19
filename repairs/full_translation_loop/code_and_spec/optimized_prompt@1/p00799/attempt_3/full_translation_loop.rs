use std::io::{self, BufRead};

const QMAX: usize = 102;

struct Battery {
    active: bool,
    time: usize,
    recharge_time: usize,
    index: usize,
    usage: Vec<usize>,
    recharge: Vec<usize>,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = match iter.next().unwrap().parse() {
            Ok(num) if num > 0 => num,
            _ => break,
        };
        let lim: usize = match iter.next().unwrap().parse() {
            Ok(num) if num > 0 => num,
            _ => break,
        };

        let mut batteries: Vec<Battery> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut usage = Vec::new();
            let mut recharge = Vec::new();

            while let Some(usage_str) = iter.next() {
                let u: usize = match usage_str.parse() {
                    Ok(num) if num > 0 => num,
                    _ => break,
                };
                let c: usize = match iter.next() {
                    Some(c_str) => match c_str.parse() {
                        Ok(num) => num,
                        _ => break,
                    },
                    None => break,
                };
                usage.push(u);
                recharge.push(c);
            }

            if !usage.is_empty() && !recharge.is_empty() {
                batteries.push(Battery {
                    active: true,
                    time: 0,
                    recharge_time: usage[0],
                    index: 0,
                    usage,
                    recharge,
                });
            }
        }

        let mut ans = 0;
        let mut queue: Vec<usize> = Vec::new();
        let mut top = 0;
        let mut end = 0;

        for t in 1..=lim {
            for (i, battery) in batteries.iter_mut().enumerate() {
                if battery.active {
                    battery.time = t;
                    if battery.recharge_time > 0 {
                        battery.recharge_time -= 1;
                    }
                    if battery.recharge_time == 0 {
                        battery.active = false;
                        battery.recharge_time = battery.recharge[battery.index];
                        queue.push(i);
                        if queue.len() > QMAX {
                            queue.remove(0); // Maintain queue size
                        }
                    }
                }
            }

            if top < queue.len() {
                for (i, battery) in batteries.iter_mut().enumerate() {
                    if battery.time != t && !battery.active {
                        if i == queue[top] {
                            battery.recharge_time -= 1;
                        } else {
                            ans += 1;
                        }
                    }
                }

                let idx = queue[top];
                let battery = &mut batteries[idx];
                if battery.recharge_time == 0 {
                    battery.active = true;
                    battery.index += 1;
                    if battery.index == battery.usage.len() {
                        battery.index = 0;
                    }
                    battery.recharge_time = battery.usage[battery.index];
                    top += 1;
                }
            }
        }
        println!("{}", ans);
    }
}
