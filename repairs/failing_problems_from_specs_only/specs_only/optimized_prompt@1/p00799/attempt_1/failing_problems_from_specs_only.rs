use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Pair {
    u: i64,
    c: i64,
}

#[derive(Clone, Copy)]
enum State {
    Active { rem_u: i64 },
    Charging { rem_c: i64 },
}

fn main() {
    // Read all input as whitespace-separated integers.
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
        let lim: i64 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };

        // Termination: n <= 0 ends input; lim is read but ignored for termination.
        if n <= 0 {
            break;
        }

        let n_usize = n as usize;

        // Read cycles for each device.
        // Limits are assumed by spec; if exceeded, behavior is unspecified.
        let mut cycles: Vec<Vec<Pair>> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let mut v: Vec<Pair> = Vec::new();
            loop {
                let u: i64 = it.next().unwrap().parse().unwrap();
                if u <= 0 {
                    break;
                }
                let c: i64 = it.next().unwrap().parse().unwrap();
                v.push(Pair { u, c });
            }
            cycles.push(v);
        }

        if lim <= 0 {
            outputs.push("0".to_string());
            continue;
        }

        // Device state.
        let mut idx_in_cycle: Vec<usize> = vec![0; n_usize];
        let mut state: Vec<State> = Vec::with_capacity(n_usize);
        for i in 0..n_usize {
            // At least one pair exists per spec.
            let p = cycles[i][0];
            state.push(State::Active { rem_u: p.u });
        }

        // Fixed-size circular buffer queue with overwrite-on-wrap semantics and no full detection.
        const QCAP: usize = 102;
        let mut qbuf: [usize; QCAP] = [0; QCAP];
        let mut qhead: usize = 0;
        let mut qtail: usize = 0;

        let mut misses: i128 = 0;

        for _t in 0..(lim as usize) {
            // Step A: advance active devices; enqueue those that enter charging.
            // Track which devices just entered charging this time unit.
            let mut just_entered: Vec<bool> = vec![false; n_usize];

            for i in 0..n_usize {
                match state[i] {
                    State::Active { rem_u } => {
                        let new_rem = rem_u - 1;
                        if new_rem == 0 {
                            // Switch to charging using current cycle pair's c.
                            let p = cycles[i][idx_in_cycle[i]];
                            state[i] = State::Charging { rem_c: p.c };
                            just_entered[i] = true;

                            // Enqueue with overwrite-on-wrap semantics.
                            qbuf[qtail % QCAP] = i;
                            qtail = qtail.wrapping_add(1);
                        } else {
                            state[i] = State::Active { rem_u: new_rem };
                        }
                    }
                    State::Charging { .. } => {
                        // Not advanced in step A.
                    }
                }
            }

            // Determine if queue is empty after step A.
            if qhead == qtail {
                continue;
            }

            // Step B: count misses among devices that were already charging before step A.
            // The serviced device for miss-counting is the queue-front device.
            let front = qbuf[qhead % QCAP];

            // Count devices in charging that did not just enter this time unit.
            let mut eligible_charging: i128 = 0;
            for i in 0..n_usize {
                if just_entered[i] {
                    continue;
                }
                if let State::Charging { .. } = state[i] {
                    eligible_charging += 1;
                }
            }

            // Every eligible charging device except the front contributes 1 miss.
            // If the front is not eligible (e.g., it just entered), then no one is excluded.
            let front_is_eligible = !just_entered[front] && matches!(state[front], State::Charging { .. });
            if front_is_eligible {
                if eligible_charging > 0 {
                    misses += eligible_charging - 1;
                }
            } else {
                misses += eligible_charging;
            }

            // Step C: apply one unit of charging service to the queue front.
            let dev = front;
            match state[dev] {
                State::Charging { rem_c } => {
                    let new_rem_c = rem_c - 1;
                    if new_rem_c == 0 {
                        // Dequeue.
                        qhead = qhead.wrapping_add(1);

                        // Switch to active, advance cycle index, set rem_u.
                        let clen = cycles[dev].len();
                        let next_idx = (idx_in_cycle[dev] + 1) % clen;
                        idx_in_cycle[dev] = next_idx;
                        let p = cycles[dev][next_idx];
                        state[dev] = State::Active { rem_u: p.u };
                    } else {
                        state[dev] = State::Charging { rem_c: new_rem_c };
                    }
                }
                State::Active { .. } => {
                    // Per spec, queue contains devices in charging; if overwritten wraparound
                    // causes inconsistency, proceed with current contents without error.
                    // No charging progress can be applied meaningfully here.
                }
            }
        }

        outputs.push(misses.to_string());
    }

    print!("{}", outputs.join("\n"));
}
