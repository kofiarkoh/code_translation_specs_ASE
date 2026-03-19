use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Active,
    Charging,
}

#[derive(Clone, Debug)]
struct Device {
    // Cyclic list of (active_duration, charging_duration)
    pairs: Vec<(i64, i64)>,
    pair_idx: usize,

    state: State,
    rem: i64, // remaining time in current state, always intended non-negative by spec
}

impl Device {
    fn new(pairs: Vec<(i64, i64)>) -> Self {
        // Spec guarantees at least one pair and positive durations.
        let (a, _) = pairs[0];
        Self {
            pairs,
            pair_idx: 0,
            state: State::Active,
            rem: a,
        }
    }

    fn cur_active(&self) -> i64 {
        self.pairs[self.pair_idx].0
    }
    fn cur_charge(&self) -> i64 {
        self.pairs[self.pair_idx].1
    }

    fn advance_pair(&mut self) {
        self.pair_idx += 1;
        if self.pair_idx >= self.pairs.len() {
            self.pair_idx = 0;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i64 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }
        let n = n as usize;

        let t_limit: i64 = it.next().unwrap().parse().unwrap();
        let t_limit_u: i64 = if t_limit < 0 { 0 } else { t_limit };

        let mut devices: Vec<Device> = Vec::with_capacity(n);

        for _ in 0..n {
            let mut pairs: Vec<(i64, i64)> = Vec::new();
            loop {
                let a: i64 = it.next().unwrap().parse().unwrap();
                if a <= 0 {
                    break; // sentinel terminates the sequence for this device
                }
                let c: i64 = it.next().unwrap().parse().unwrap();
                // Spec: durations are positive integers.
                pairs.push((a, c));
            }
            devices.push(Device::new(pairs));
        }

        // Queue of devices waiting for service, in order they entered charging.
        let mut queue: VecDeque<usize> = VecDeque::new();
        // Currently serviced device index, if any.
        let mut serviced: Option<usize> = None;

        let mut total_unserviced: i128 = 0;

        for _time in 0..t_limit_u {
            // If no one is being serviced, pick next waiting device (if any).
            if serviced.is_none() {
                serviced = queue.pop_front();
            }

            // Count charging devices and add unserviced charging devices for this time unit.
            let mut charging_count: i64 = 0;
            for d in &devices {
                if d.state == State::Charging {
                    charging_count += 1;
                }
            }
            let serviced_present = if serviced.is_some() { 1i64 } else { 0i64 };
            let unserviced_this = charging_count - serviced_present;
            if unserviced_this > 0 {
                total_unserviced += unserviced_this as i128;
            }

            // Apply per-time-unit decrements.
            // All devices decrement by 1 in their current state.
            // Additionally, the serviced charging device decrements by an extra 1.
            for (idx, dev) in devices.iter_mut().enumerate() {
                if dev.rem > 0 {
                    dev.rem -= 1;
                }
                if dev.state == State::Charging {
                    if let Some(sidx) = serviced {
                        if sidx == idx && dev.rem > 0 {
                            dev.rem -= 1;
                        }
                    }
                }
            }

            // Handle transitions. When a device completes active, it enters charging and is enqueued.
            // When a device completes charging, it enters active and advances to next pair.
            // If the serviced device completes charging, service ends and next device can be chosen
            // in subsequent time units (i.e., not immediately within the same time unit).
            let mut serviced_completed = false;

            for (idx, dev) in devices.iter_mut().enumerate() {
                if dev.rem > 0 {
                    continue;
                }

                match dev.state {
                    State::Active => {
                        // Active -> Charging
                        dev.state = State::Charging;
                        dev.rem = dev.cur_charge();
                        queue.push_back(idx);
                    }
                    State::Charging => {
                        // Charging -> Active
                        dev.state = State::Active;
                        dev.advance_pair();
                        dev.rem = dev.cur_active();

                        if let Some(sidx) = serviced {
                            if sidx == idx {
                                serviced_completed = true;
                            }
                        }
                    }
                }
            }

            if serviced_completed {
                serviced = None;
            }
        }

        outputs.push(total_unserviced.to_string());
    }

    print!("{}", outputs.join("\n"));
}
