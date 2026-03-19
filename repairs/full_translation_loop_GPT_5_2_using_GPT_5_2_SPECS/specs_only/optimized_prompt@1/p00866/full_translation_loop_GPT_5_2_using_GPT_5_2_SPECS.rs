use std::io::{self, Read};

const SECS_12H: usize = 12 * 60 * 60; // 43200

fn fmt_time(t: usize) -> String {
    let h = t / 3600;
    let m = (t / 60) % 60;
    let s = t % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Precompute, for each time t in [0, 43199], the triple of hand positions
    // in "minutes/seconds units" (i.e., 0..59) for hour/minute/second hands.
    //
    // We interpret the hour hand position as the minute-mark it points to:
    // floor((t mod 12h) / 720) in 0..59 (since 12h / 60 = 720s per mark).
    // Minute hand: floor((t mod 1h) / 60) in 0..59.
    // Second hand: t mod 60 in 0..59.
    //
    // This matches the spec's "minutes/seconds units" and stays within 0..59.
    let mut hpos = [0u8; SECS_12H];
    let mut mpos = [0u8; SECS_12H];
    let mut spos = [0u8; SECS_12H];
    for t in 0..SECS_12H {
        hpos[t] = ((t / 720) % 60) as u8;
        mpos[t] = ((t / 60) % 60) as u8;
        spos[t] = (t % 60) as u8;
    }

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n <= 0 {
            break;
        }
        let n = n as usize;

        // Fixed limits are part of the spec; if violated, behavior is undefined.
        // We still try to proceed without panicking.
        let mut watches: Vec<[u8; 3]> = Vec::with_capacity(n);
        for _ in 0..n {
            let a: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
            let b: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
            let c: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
            // Values are intended 0..59; we clamp via modulo to keep in-range
            // without inventing error handling.
            let aa = ((a % 60 + 60) % 60) as u8;
            let bb = ((b % 60 + 60) % 60) as u8;
            let cc = ((c % 60 + 60) % 60) as u8;
            watches.push([aa, bb, cc]);
        }

        // For each watch, compute feasible times within the 12-hour cycle.
        // Model: observed triple are the three hands in unknown assignment, and
        // there is an unknown common offset k (0..59) added to all three hand
        // positions modulo 60 (a "stopped" watch with a common shift).
        //
        // A time t is feasible if there exists a permutation p of (H,M,S) and
        // an offset k such that:
        //   (pos_p0(t) + k) % 60 == obs0
        //   (pos_p1(t) + k) % 60 == obs1
        //   (pos_p2(t) + k) % 60 == obs2
        //
        // Equivalently, for a given permutation, k is determined by the first
        // equation; then the other two must match.
        let perms: [[usize; 3]; 6] = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];

        // feasible[w][t] = true if time t is feasible for watch w.
        let mut feasible: Vec<Vec<bool>> = Vec::with_capacity(n);
        for w in 0..n {
            let obs = watches[w];
            let mut ok = vec![false; SECS_12H];

            for t in 0..SECS_12H {
                let pos = [hpos[t], mpos[t], spos[t]];
                let mut any = false;

                for p in perms.iter() {
                    // k = obs0 - pos[p0] mod 60
                    let k = (obs[0] as i16 - pos[p[0]] as i16).rem_euclid(60) as u8;

                    let v1 = (pos[p[1]] as u16 + k as u16) % 60;
                    if v1 as u8 != obs[1] {
                        continue;
                    }
                    let v2 = (pos[p[2]] as u16 + k as u16) % 60;
                    if v2 as u8 != obs[2] {
                        continue;
                    }
                    any = true;
                    break;
                }

                ok[t] = any;
            }

            feasible.push(ok);
        }

        // Find the minimum-length interval [L, R] (inclusive) such that for each
        // watch w, there exists some t in [L, R] with feasible[w][t] == true.
        //
        // Use a two-pointer sliding window over time 0..43199.
        let mut counts = vec![0i32; n];
        let mut covered = 0usize;

        let mut best_l = 0usize;
        let mut best_r = SECS_12H - 1;
        let mut best_len = best_r - best_l;

        let mut l = 0usize;
        for r in 0..SECS_12H {
            for w in 0..n {
                if feasible[w][r] {
                    if counts[w] == 0 {
                        covered += 1;
                    }
                    counts[w] += 1;
                }
            }

            while covered == n && l <= r {
                let len = r - l;
                if len < best_len || (len == best_len && l < best_l) {
                    best_len = len;
                    best_l = l;
                    best_r = r;
                }

                for w in 0..n {
                    if feasible[w][l] {
                        counts[w] -= 1;
                        if counts[w] == 0 {
                            covered -= 1;
                        }
                    }
                }
                l += 1;
            }
        }

        out.push_str(&format!("{} {}\n", fmt_time(best_l), fmt_time(best_r)));
    }

    print!("{out}");
}
