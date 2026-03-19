use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Battery {
    // Narrowest reasonable interpretation:
    // Each battery provides a repeating cycle: "usage time" then "recharge time".
    // While active, it can be used once per time unit; after `use_time` uses, it becomes inactive
    // and is scheduled to become active again after `recharge_time` time units.
    use_time: i64,
    recharge_time: i64,
    remaining_active_uses: i64,
    active: bool,
}

fn main() {
    // Input format is underspecified; we implement a robust token-based parser.
    // If input is malformed, behavior is unspecified; we simply stop when parsing fails.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next().and_then(|s| s.parse::<i64>().ok());
        let n = match n_opt {
            Some(v) => v,
            None => break,
        };

        // Terminate gracefully when a non-positive integer is encountered for number of batteries.
        if n <= 0 {
            break;
        }

        let t_opt = it.next().and_then(|s| s.parse::<i64>().ok());
        let time_limit = match t_opt {
            Some(v) => v,
            None => break,
        };

        // Preconditions say time limit is positive; if not, we still simulate zero/negative time
        // as producing zero uses (narrowest reasonable behavior).
        let mut batteries: Vec<Battery> = Vec::with_capacity(n as usize);

        for _ in 0..n {
            // Spec says: "sequence of positive integers representing usage times followed by corresponding recharge times"
            // but does not define how many pairs per battery. Narrowest reasonable interpretation:
            // exactly one usage time and one recharge time per battery.
            let use_time = match it.next().and_then(|s| s.parse::<i64>().ok()) {
                Some(v) => v,
                None => {
                    // Malformed/incomplete input: stop processing further.
                    print_outputs(outputs);
                    return;
                }
            };
            let recharge_time = match it.next().and_then(|s| s.parse::<i64>().ok()) {
                Some(v) => v,
                None => {
                    print_outputs(outputs);
                    return;
                }
            };

            // Preconditions say positive; if not, keep values as-is to avoid inventing behavior.
            batteries.push(Battery {
                use_time,
                recharge_time,
                remaining_active_uses: use_time,
                active: true,
            });
        }

        // Queue of (time_when_recharged, battery_index).
        // Invariant: order reflects when batteries become available again.
        let mut recharge_queue: VecDeque<(i64, usize)> = VecDeque::new();

        // Total number of battery usages that occurred without a corresponding recharge within the time limit.
        // Interpretation: count each usage event that happens while the battery has not yet been recharged
        // after its last depletion. This is simply the number of usage events executed during the simulation.
        // (A "corresponding recharge within the time frame" would only matter for uses that happen near the end,
        // but the spec asks for "usages ... without ... recharge within the given time frame", which matches
        // counting uses whose subsequent recharge time is > time_limit. However, the spec also says "tracks usage
        // and recharge cycles" and "calculates total number of times batteries are used without being recharged
        // within the given time frame". Narrowest consistent interpretation:
        // count uses whose *next recharge completion* occurs after the time limit (or never within it).
        let mut count_without_recharge_within_limit: i64 = 0;

        // Simulate discrete time steps from 1..=time_limit (if positive).
        // At each time unit:
        // 1) Apply any recharges that complete at this time.
        // 2) Use all currently active batteries once (one usage per active battery per time unit).
        //    If a battery depletes, schedule its recharge completion.
        // This preserves a consistent queue order and battery state transitions.
        let tmax = time_limit;
        if tmax > 0 {
            for t in 1..=tmax {
                // Process recharges that complete now.
                while let Some(&(ready_t, idx)) = recharge_queue.front() {
                    if ready_t != t {
                        break;
                    }
                    recharge_queue.pop_front();
                    let b = &mut batteries[idx];
                    b.active = true;
                    b.remaining_active_uses = b.use_time;
                }

                // Use each active battery once.
                for (idx, b) in batteries.iter_mut().enumerate() {
                    if !b.active {
                        continue;
                    }

                    // Perform one usage.
                    // If use_time <= 0, remaining_active_uses may be <= 0; we still decrement to avoid inventing behavior.
                    b.remaining_active_uses -= 1;

                    // Determine when the corresponding recharge would complete if the battery depletes now.
                    // We count this usage as "without recharge within time limit" if the recharge completion time
                    // is after the time limit (or if recharge_time <= 0, it may complete immediately or in the past).
                    // Narrowest: only count as "without recharge within limit" when depletion occurs and the recharge
                    // completion is strictly greater than time_limit.
                    if b.remaining_active_uses == 0 {
                        b.active = false;
                        let ready_t = t.saturating_add(b.recharge_time);
                        // Maintain queue order by time; since we process time in increasing order and only push
                        // ready_t >= current time for positive recharge_time, FIFO is consistent.
                        // If recharge_time is non-positive, ready_t may be <= t; we still enqueue and it will be
                        // processed when its time matches (which may never happen). This matches "undefined behavior"
                        // territory without inventing fixes.
                        recharge_queue.push_back((ready_t, idx));

                        if ready_t > time_limit {
                            // All uses that lead to a recharge after the limit are counted.
                            // This counts the *depleting* usage only, which is the narrowest mapping to
                            // "usage without being recharged within the time frame".
                            count_without_recharge_within_limit += 1;
                        }
                    }
                }
            }
        }

        outputs.push(count_without_recharge_within_limit.to_string());
    }

    print_outputs(outputs);
}

fn print_outputs(outputs: Vec<String>) {
    if outputs.is_empty() {
        return;
    }
    // One integer per test case, each on its own line.
    print!("{}", outputs.join("\n"));
}
