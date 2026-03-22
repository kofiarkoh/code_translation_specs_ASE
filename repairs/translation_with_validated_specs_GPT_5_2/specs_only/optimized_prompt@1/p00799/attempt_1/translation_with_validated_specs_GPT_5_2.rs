use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Pair {
    u: i64,
    c: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Active,
    Charging,
}

struct Device {
    cycle: Vec<Pair>,
    idx: usize,      // current cycle index
    state: State,
    rem: i64,        // remaining time in current state
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
        let lim: i64 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };

        if n <= 0 {
            // Termination: lim is read but ignored for termination purposes.
            break;
        }

        let n_usize = n as usize;

        let mut devices: Vec<Device> = Vec::with_capacity(n_usize);

        for _ in 0..n_usize {
            let mut cycle: Vec<Pair> = Vec::new();
            loop {
                let u: i64 = it.next().unwrap().parse().unwrap();
                if u <= 0 {
                    break;
                }
                let c: i64 = it.next().unwrap().parse().unwrap();
                // Spec says all used u and c are positive; keep as-is.
                cycle.push(Pair { u, c });
                // If input exceeds built-in limits, behavior unspecified; we do not add checks.
            }

            // Spec: at least one pair must be provided for each device.
            let first = cycle[0];
            devices.push(Device {
                cycle,
                idx: 0,
                state: State::Active,
                rem: first.u,
            });
        }

        if lim <= 0 {
            outputs.push("0".to_string());
            continue;
        }

        // Fixed-size circular buffer queue of capacity 102, with overwrite-on-wrap semantics.
        const QCAP: usize = 102;
        let mut qbuf: [usize; QCAP] = [0; QCAP];
        let mut qhead: usize = 0; // index of front element
        let mut qtail: usize = 0; // index where next enqueue writes
        // No full-queue detection; overwrite happens naturally when qtail laps qhead.
        // Emptiness is represented by qhead == qtail.
        // This matches a common C circular buffer pattern with no full detection.

        let mut misses: i128 = 0;

        for _t in 1..=lim {
            // Track which devices just entered charging during step (A).
            let mut just_entered: Vec<bool> = vec![false; n_usize];

            // (A) Advance active devices and enqueue new chargers.
            for i in 0..n_usize {
                if devices[i].state == State::Active {
                    devices[i].rem -= 1;
                    if devices[i].rem == 0 {
                        devices[i].state = State::Charging;
                        let c = devices[i].cycle[devices[i].idx].c;
                        devices[i].rem = c;
                        just_entered[i] = true;

                        // Enqueue with overwrite-on-wrap semantics.
                        qbuf[qtail % QCAP] = i;
                        qtail = qtail.wrapping_add(1);
                        // Note: wrapping_add preserves modulo-2^usize behavior; indexing uses % QCAP.
                        // This emulates C unsigned wraparound for long runs.
                    }
                }
            }

            // Determine if queue is empty after (A).
            let empty = qhead == qtail;

            if empty {
                continue;
            }

            // Front device for this time unit (for miss-counting and service).
            let front = qbuf[qhead % QCAP];

            // (B) Count misses among devices that were already charging before step (A).
            // We interpret "already charging before step (A)" as: charging now AND not just_entered.
            // Among those, all except the queue-front contribute 1 miss.
            for i in 0..n_usize {
                if devices[i].state == State::Charging && !just_entered[i] && i != front {
                    misses += 1;
                }
            }

            // (C) Apply one unit of charging service to the queue front.
            {
                let fi = front;
                devices[fi].rem -= 1;
                if devices[fi].rem == 0 {
                    // Dequeue
                    qhead = qhead.wrapping_add(1);

                    // Switch to active, advance cycle index, set remaining active time.
                    devices[fi].state = State::Active;
                    let next_idx = (devices[fi].idx + 1) % devices[fi].cycle.len();
                    devices[fi].idx = next_idx;
                    devices[fi].rem = devices[fi].cycle[next_idx].u;
                } else {
                    // Remains at front; queue indices unchanged.
                }
            }
        }

        outputs.push(misses.to_string());
    }

    print!("{}", outputs.join("\n"));
}
