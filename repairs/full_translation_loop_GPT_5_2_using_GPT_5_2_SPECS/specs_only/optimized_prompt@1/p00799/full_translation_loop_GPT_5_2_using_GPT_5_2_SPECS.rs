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
    rem: i64, // remaining time in current state
}

impl Device {
    fn new(pairs: Vec<(i64, i64)>) -> Self {
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

    let mut outs: Vec<String> = Vec::new();

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: i64 = n_tok.parse().unwrap();
        if n <= 0 {
            break;
        }
        let n = n as usize;

        let t_limit: i64 = it.next().unwrap().parse().unwrap();
        let t_limit = if t_limit < 0 { 0 } else { t_limit };

        let mut devices: Vec<Device> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut pairs: Vec<(i64, i64)> = Vec::new();
            loop {
                let a: i64 = it.next().unwrap().parse().unwrap();
                if a <= 0 {
                    break;
                }
                let c: i64 = it.next().unwrap().parse().unwrap();
                pairs.push((a, c));
            }
            devices.push(Device::new(pairs));
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut serviced: Option<usize> = None;

        let mut total: i128 = 0;

        for _ in 0..t_limit {
            // Choose a serviced device for this time unit if none is currently serviced.
            if serviced.is_none() {
                serviced = queue.pop_front();
            }

            // Count unserviced charging devices for this time unit based on the state
            // at the start of the time unit (before decrements/transitions).
            let mut charging_count: i64 = 0;
            for d in &devices {
                if d.state == State::Charging {
                    charging_count += 1;
                }
            }
            let unserviced = charging_count - if serviced.is_some() { 1 } else { 0 };
            if unserviced > 0 {
                total += unserviced as i128;
            }

            // Decrement all devices by 1 for being in their current state.
            for d in devices.iter_mut() {
                if d.rem > 0 {
                    d.rem -= 1;
                }
            }

            // Additional decrement for the serviced charging device.
            if let Some(sidx) = serviced {
                let d = &mut devices[sidx];
                if d.state == State::Charging && d.rem > 0 {
                    d.rem -= 1;
                }
            }

            // Apply transitions after all decrements for this time unit.
            // If the serviced device finishes charging, service ends now; the next device
            // (if any) will only be selected at the start of the next time unit.
            let mut serviced_finished = false;

            for (idx, d) in devices.iter_mut().enumerate() {
                if d.rem != 0 {
                    continue;
                }
                match d.state {
                    State::Active => {
                        d.state = State::Charging;
                        d.rem = d.cur_charge();
                        queue.push_back(idx);
                    }
                    State::Charging => {
                        d.state = State::Active;
                        d.advance_pair();
                        d.rem = d.cur_active();
                        if serviced == Some(idx) {
                            serviced_finished = true;
                        }
                    }
                }
            }

            if serviced_finished {
                serviced = None;
            }
        }

        outs.push(total.to_string());
    }

    print!("{}", outs.join("\n"));
}
