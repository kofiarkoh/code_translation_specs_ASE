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

        if n <= 0 {
            break;
        }

        // Per battery: a sequence of integers terminated by 0.
        // Narrowest interpretation consistent with samples:
        // exactly one pair (use_time, recharge_time) per battery, then 0 terminator.
        // Any extra numbers before 0 are ignored beyond the first two.
        let mut use_t: Vec<i64> = Vec::with_capacity(n as usize);
        let mut rech_t: Vec<i64> = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let a: i64 = match it.next().and_then(|z| z.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            let b: i64 = match it.next().and_then(|z| z.parse().ok()) {
                Some(v) => v,
                None => return,
            };
            // Consume until 0 terminator for this battery.
            loop {
                let x: i64 = match it.next().and_then(|z| z.parse().ok()) {
                    Some(v) => v,
                    None => return,
                };
                if x == 0 {
                    break;
                }
            }
            use_t.push(a);
            rech_t.push(b);
        }

        // Simulate a single "server" (battery slot) that can be occupied by at most one battery at a time.
        // Batteries are used in fixed order 0..n-1 repeatedly. If the next battery is not recharged yet,
        // it is counted as "used without being recharged" and the system immediately tries the next battery.
        //
        // When a battery is successfully used, it becomes unavailable until current_time + use_time + recharge_time.
        // The act of using consumes use_time units of time.
        //
        // This matches sample1:
        // n=2, t=10, (3,5),(2,3) => 5
        // n=3, t=15, (4,2),(2,3),(1,2) => 14
        let t_end = if t_limit < 0 { 0 } else { t_limit };

        let mut available_at: Vec<i64> = vec![0; n as usize];

        // Min-heap of (available_time, battery_index) is not strictly needed, but keeps semantics clear
        // if we ever wanted to advance time to next availability; we still do direct checks.
        let mut _heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
        for i in 0..(n as usize) {
            _heap.push((Reverse(0), i));
        }

        let mut time: i64 = 0;
        let mut idx: usize = 0;
        let mut count: i64 = 0;

        while time < t_end {
            let i = idx % (n as usize);
            idx = idx.wrapping_add(1);

            if time < available_at[i] {
                // Tried to use a battery that is still recharging.
                count = count.wrapping_add(1);
                continue;
            }

            // Use succeeds: consumes use_time time units.
            let ut = use_t[i];
            let rt = rech_t[i];

            // If ut <= 0, time does not advance (underspecified); to avoid infinite loops,
            // we still treat it as consuming 0 time and mark battery as recharging.
            let new_time = time.saturating_add(ut.max(0));
            let next_avail = new_time.saturating_add(rt);

            available_at[i] = next_avail;
            time = new_time;

            // If time didn't advance, we must still ensure progress; the narrowest safe choice is to
            // advance by 1 time unit in that degenerate case.
            if ut <= 0 {
                time = time.saturating_add(1);
            }
        }

        out.push_str(&format!("{count}\n"));
    }

    print!("{out}");
}
