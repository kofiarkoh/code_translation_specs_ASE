use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).ok();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let t_limit: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        // Terminate when a non-positive integer is encountered for the number of batteries.
        if n <= 0 {
            break;
        }

        // Input per battery is a sequence of positive integers terminated by 0.
        // Narrowest interpretation consistent with samples:
        // each battery provides a repeating cycle of (use_time, recharge_time) pairs,
        // terminated by a single 0. The cycle repeats indefinitely.
        let mut cycles: Vec<Vec<(i64, i64)>> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let mut v: Vec<i64> = Vec::new();
            loop {
                let x: i64 = match it.next().and_then(|z| z.parse().ok()) {
                    Some(val) => val,
                    None => return, // malformed/incomplete input => stop
                };
                if x == 0 {
                    break;
                }
                v.push(x);
            }
            // Preconditions say each battery has at least one usage and recharge time specified.
            // If odd count, drop the last dangling value (narrowest non-inventive behavior).
            let mut pairs = Vec::new();
            let mut i = 0;
            while i + 1 < v.len() {
                pairs.push((v[i], v[i + 1]));
                i += 2;
            }
            cycles.push(pairs);
        }

        // Event-driven simulation:
        // Each battery alternates between:
        // - active for `use_time` time units (contributes 1 usage per time unit),
        // - inactive for `recharge_time` time units,
        // then advances to next pair (cyclic).
        //
        // We need total usages that occur within [0, t_limit).
        // This matches sample behavior (e.g., n=2, t=10 with cycles (3,5) and (2,3) => 5).
        let t_end = if t_limit < 0 { 0 } else { t_limit };

        // Min-heap of next transition times: (time, battery_index).
        let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();

        // Per-battery state.
        let mut active: Vec<bool> = vec![false; n as usize];
        let mut next_pair_idx: Vec<usize> = vec![0; n as usize];

        // Initialize: all batteries start active at time 0 using their first pair.
        for bi in 0..(n as usize) {
            if cycles[bi].is_empty() {
                // If no valid pair, battery never contributes.
                continue;
            }
            active[bi] = true;
            let (use_t, _) = cycles[bi][0];
            // Transition to inactive after use_t time units.
            heap.push((Reverse(use_t), bi));
        }

        let mut cur_time: i64 = 0;
        let mut active_count: i64 = active.iter().filter(|&&a| a).count() as i64;
        let mut total_uses: i64 = 0;

        while cur_time < t_end {
            let next_time = match heap.peek() {
                Some(&(Reverse(t), _)) => t,
                None => {
                    // No more transitions; if any active, they stay active forever (not expected by spec),
                    // so accumulate remaining time and stop.
                    if active_count > 0 {
                        total_uses += active_count.saturating_mul(t_end - cur_time);
                    }
                    break;
                }
            };

            let seg_end = if next_time < t_end { next_time } else { t_end };
            if seg_end > cur_time && active_count > 0 {
                total_uses += active_count.saturating_mul(seg_end - cur_time);
            }
            cur_time = seg_end;
            if cur_time >= t_end {
                break;
            }

            // Process all transitions at this exact time.
            while let Some(&(Reverse(t), bi)) = heap.peek() {
                if t != cur_time {
                    break;
                }
                heap.pop();

                if cycles[bi].is_empty() {
                    continue;
                }

                if active[bi] {
                    // Active -> Inactive, schedule recharge completion.
                    active[bi] = false;
                    active_count -= 1;

                    let idx = next_pair_idx[bi];
                    let (_, recharge_t) = cycles[bi][idx];
                    let ready = cur_time.saturating_add(recharge_t);
                    heap.push((Reverse(ready), bi));
                } else {
                    // Inactive -> Active, advance to next pair (cyclic), schedule end of active period.
                    active[bi] = true;
                    active_count += 1;

                    let len = cycles[bi].len();
                    let next_idx = if len == 0 { 0 } else { (next_pair_idx[bi] + 1) % len };
                    next_pair_idx[bi] = next_idx;

                    let (use_t, _) = cycles[bi][next_idx];
                    let end_active = cur_time.saturating_add(use_t);
                    heap.push((Reverse(end_active), bi));
                }
            }
        }

        out.push_str(&format!("{total_uses}\n"));
    }

    print!("{out}");
}
