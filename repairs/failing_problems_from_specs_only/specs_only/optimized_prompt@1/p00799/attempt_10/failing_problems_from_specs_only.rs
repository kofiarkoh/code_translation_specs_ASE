use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Pair {
    u: i32,
    c: i32,
}

#[derive(Clone, Copy)]
enum State {
    Active { rem_u: i32 },
    Charging { rem_c: i32 },
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outs: Vec<String> = Vec::new();

    loop {
        let Some(ns) = it.next() else { break };
        let n_in: i32 = ns.parse().unwrap();
        let lim: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };

        if n_in <= 0 {
            break;
        }
        let n = n_in as usize;

        let mut cycles: Vec<Vec<Pair>> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut v: Vec<Pair> = Vec::new();
            loop {
                let u: i32 = it.next().unwrap().parse().unwrap();
                if u <= 0 {
                    break;
                }
                let c: i32 = it.next().unwrap().parse().unwrap();
                v.push(Pair { u, c });
            }
            cycles.push(v);
        }

        if lim <= 0 {
            outs.push("0".to_string());
            continue;
        }

        let mut idx_in_cycle = vec![0usize; n];
        let mut state: Vec<State> = Vec::with_capacity(n);
        for i in 0..n {
            let p = cycles[i][0];
            state.push(State::Active { rem_u: p.u });
        }

        // Fixed-size circular buffer queue with overwrite-on-wrap semantics and no full detection.
        const QCAP: usize = 102;
        let mut qbuf: [usize; QCAP] = [0; QCAP];
        let mut qhead: usize = 0;
        let mut qtail: usize = 0;

        let mut misses: i64 = 0;

        for _ in 0..(lim as usize) {
            // Snapshot which devices were charging at the start of this time unit.
            let mut was_charging = vec![false; n];
            for i in 0..n {
                was_charging[i] = matches!(state[i], State::Charging { .. });
            }

            // Step A: advance active devices; enqueue those that enter charging.
            for i in 0..n {
                if let State::Active { rem_u } = state[i] {
                    let new_rem = rem_u - 1;
                    if new_rem == 0 {
                        let p = cycles[i][idx_in_cycle[i]];
                        state[i] = State::Charging { rem_c: p.c };

                        qbuf[qtail % QCAP] = i;
                        qtail = qtail.wrapping_add(1);
                    } else {
                        state[i] = State::Active { rem_u: new_rem };
                    }
                }
            }

            if qhead == qtail {
                continue;
            }

            // Step B: count misses among devices that were already charging before step A.
            // The queue-front device is excluded from misses even if it was not already charging.
            let front = qbuf[qhead % QCAP];

            let mut eligible: i64 = 0;
            for i in 0..n {
                if was_charging[i] {
                    eligible += 1;
                }
            }

            // Misses are "eligible devices excluding the serviced device", but if the serviced device
            // is not in the eligible set, nothing is excluded.
            if was_charging[front] {
                if eligible > 0 {
                    misses += eligible - 1;
                }
            } else {
                misses += eligible;
            }

            // Step C: apply one unit of charging service to the queue front.
            let dev = front;
            if let State::Charging { rem_c } = state[dev] {
                let new_rem_c = rem_c - 1;
                if new_rem_c == 0 {
                    qhead = qhead.wrapping_add(1);

                    let clen = cycles[dev].len();
                    let next_idx = (idx_in_cycle[dev] + 1) % clen;
                    idx_in_cycle[dev] = next_idx;
                    let p = cycles[dev][next_idx];
                    state[dev] = State::Active { rem_u: p.u };
                } else {
                    state[dev] = State::Charging { rem_c: new_rem_c };
                }
            } else {
                // Overwrite-on-wrap may corrupt queue contents; proceed without error.
            }
        }

        outs.push(misses.to_string());
    }

    print!("{}", outs.join("\n"));
}
