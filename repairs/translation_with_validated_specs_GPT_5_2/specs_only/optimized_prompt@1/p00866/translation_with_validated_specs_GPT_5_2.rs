use std::cmp::Ordering;
use std::io::{self, Read};

const DAY_SECONDS: usize = 12 * 60 * 60; // 0..=43199
const MOD: i32 = 60;

fn mod60(x: i32) -> i32 {
    let r = x % MOD;
    if r < 0 { r + MOD } else { r }
}

fn fmt_time(t: usize) -> String {
    let h = t / 3600;
    let m = (t % 3600) / 60;
    let s = t % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn next_permutation(a: &mut [i32; 3]) -> bool {
    // Lexicographic next_permutation for length 3.
    // Returns false if it was the last permutation (and leaves it in last state).
    let n = 3;
    let mut i = n as i32 - 2;
    while i >= 0 && a[i as usize] >= a[(i + 1) as usize] {
        i -= 1;
    }
    if i < 0 {
        return false;
    }
    let i = i as usize;
    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

fn feasible_times_for_watch(r0: i32, r1: i32, r2: i32) -> Vec<usize> {
    // A time t (h,m,s) is feasible for this watch if there exists:
    // - a permutation assigning the three readings to (hour_pos, minute_pos, second_pos) on 0..59
    // - an integer offset k in 0..59 added uniformly to all three readings (mod 60)
    // such that:
    //   minute_pos == m
    //   second_pos == s
    //   hour_pos == (5*h + m/12) mod 60, with integer division (standard analog geometry discretized)
    //
    // We interpret "hour-hand advances with minutes" on a 0..59 scale as:
    //   hour_pos = floor((60*(h + m/60))/12) = 5*h + floor(m/12)
    // This is the narrowest integer-consistent interpretation implied by the spec.
    let mut times = Vec::new();

    // Generate all 6 permutations of the three readings.
    let mut p = [r0, r1, r2];
    p.sort(); // start from lexicographically smallest to have deterministic search order
    loop {
        let a = p[0];
        let b = p[1];
        let c = p[2];

        // For each offset k, the displayed positions become (a+k, b+k, c+k) mod 60.
        // We need b' = m, c' = s, and a' = hour_pos(h,m).
        for k in 0..60 {
            let hour_pos = mod60(a + k);
            let m = mod60(b + k) as usize;
            let s = mod60(c + k) as usize;

            // For each hour h in 0..11, check if hour_pos matches 5*h + floor(m/12).
            let m_i32 = m as i32;
            let hour_from_m = (m_i32 / 12) as i32; // floor
            for h in 0..12 {
                let expected = mod60(5 * (h as i32) + hour_from_m);
                if expected == hour_pos {
                    let t = (h as usize) * 3600 + m * 60 + s;
                    times.push(t);
                }
            }
        }

        if !next_permutation(&mut p) {
            break;
        }
    }

    times.sort_unstable();
    times.dedup();
    times
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        if n <= 0 {
            break;
        }
        let n = n as usize;

        // For each time t, store how many watches can be satisfied at t.
        // We will later find the minimal interval [L,R] such that for every watch i,
        // there exists some t in [L,R] feasible for i.
        //
        // Approach:
        // - Build for each watch its sorted feasible times list.
        // - Use a sliding window over the global time axis 0..43199:
        //   Maintain counts per watch of how many feasible times are currently in window.
        //   We can update by adding/removing time points using a time->list-of-watches mapping.
        //
        // To avoid O(n*DAY_SECONDS), we build an event list:
        //   events[t] = list of watch indices that are feasible at time t.
        let mut events: Vec<Vec<usize>> = vec![Vec::new(); DAY_SECONDS];

        for wi in 0..n {
            let r0: i32 = it.next().unwrap().parse().unwrap();
            let r1: i32 = it.next().unwrap().parse().unwrap();
            let r2: i32 = it.next().unwrap().parse().unwrap();

            let times = feasible_times_for_watch(r0, r1, r2);
            // Spec assumes at least one feasible time exists; otherwise behavior unspecified.
            for &t in &times {
                events[t].push(wi);
            }
        }

        let mut in_window = vec![0u32; n];
        let mut covered = 0usize;

        let mut best_l = 0usize;
        let mut best_r = DAY_SECONDS - 1;
        let mut best_len = best_r - best_l;

        let mut l = 0usize;
        for r in 0..DAY_SECONDS {
            for &wi in &events[r] {
                if in_window[wi] == 0 {
                    covered += 1;
                }
                in_window[wi] += 1;
            }

            while covered == n && l <= r {
                let len = r - l;
                // Tie-breaking: earliest by start time among those encountered by search order.
                // Our search order is increasing r, and for each r we shrink l as much as possible,
                // so the first time we see a given (len, l) is deterministic.
                if len < best_len || (len == best_len && l < best_l) {
                    best_len = len;
                    best_l = l;
                    best_r = r;
                }

                for &wi in &events[l] {
                    in_window[wi] -= 1;
                    if in_window[wi] == 0 {
                        covered -= 1;
                    }
                }
                l += 1;
            }
        }

        out.push_str(&fmt_time(best_l));
        out.push(' ');
        out.push_str(&fmt_time(best_r));
        out.push('\n');
    }

    print!("{out}");
}
