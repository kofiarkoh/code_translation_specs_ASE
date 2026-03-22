use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Transitions: each node has exactly one outgoing edge.
    // The specification does not define whether indices are 0-based or 1-based.
    // We choose the narrowest reasonable interpretation commonly used in such problems:
    // input is 1-based; convert to 0-based.
    let mut next = vec![0usize; n];
    for i in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        next[i] = (v - 1) as usize;
    }

    // Scores per node (i64).
    let mut score = vec![0i64; n];
    for i in 0..n {
        score[i] = it.next().unwrap().parse().unwrap();
    }

    // For each start node, follow the functional graph until a cycle is detected.
    // Compute the maximum prefix sum achievable with 1..=k moves.
    let mut global_best: i64 = i64::MIN;

    for start in 0..n {
        // Map node -> first step index when visited in this walk (0-based step index).
        let mut first_seen = vec![-1i32; n];

        // Sequence of landed nodes (destinations of each move) and their scores.
        let mut landed_nodes: Vec<usize> = Vec::new();
        let mut landed_scores: Vec<i64> = Vec::new();

        let mut cur = start;
        let mut step: i32 = 0;

        // Build the walk until we revisit a node (cycle found).
        loop {
            let nxt = next[cur];
            cur = nxt;

            if first_seen[cur] != -1 {
                // Cycle begins at first_seen[cur] in the landed sequence.
                break;
            }
            first_seen[cur] = step;
            landed_nodes.push(cur);
            landed_scores.push(score[cur]);
            step += 1;

            // In a functional graph, a cycle must appear within n steps.
            if step as usize >= n {
                break;
            }
        }

        // Determine cycle start index and cycle length.
        let cycle_start = first_seen[cur] as usize;
        let cycle_len = landed_nodes.len() - cycle_start;

        // Prefix sums over landed_scores: pref[i] = sum of first i scores (i from 0..=m).
        let m = landed_scores.len();
        let mut pref = vec![0i64; m + 1];
        for i in 0..m {
            pref[i + 1] = pref[i] + landed_scores[i];
        }

        // Best within the pre-cycle part (including possibly some of the cycle before repeating),
        // but limited by k moves.
        let max_direct = std::cmp::min(k as usize, m);
        for t in 1..=max_direct {
            global_best = global_best.max(pref[t]);
        }

        // If k exceeds m, we can take additional moves by looping the cycle.
        if k as usize > m && cycle_len > 0 {
            let cycle_sum = pref[cycle_start + cycle_len] - pref[cycle_start];

            // For each possible remainder r (number of steps taken into the cycle after reaching it),
            // compute the best total using as many full cycles as allowed.
            //
            // Total steps = cycle_start + r + q * cycle_len, with 0 <= r < cycle_len, q >= 0.
            // Must satisfy total steps <= k and total steps >= 1.
            //
            // Base sum for reaching cycle_start + r steps:
            // base = pref[cycle_start + r]
            //
            // Then add q * cycle_sum.
            //
            // If cycle_sum > 0, take maximum q; otherwise take q = 0.
            for r in 0..cycle_len {
                let base_steps = cycle_start + r;
                if base_steps == 0 {
                    // Zero moves are not allowed; but base_steps==0 implies r==0 and cycle_start==0.
                    // We'll handle totals with at least one move via q or by considering r>0 elsewhere.
                }
                if (base_steps as i64) > k {
                    continue;
                }

                let remaining = k - base_steps as i64;
                let max_q = remaining / cycle_len as i64;

                let base_sum = pref[base_steps];

                if cycle_sum > 0 {
                    // Taking more full cycles increases the sum.
                    // Ensure at least one move overall.
                    let q = max_q;
                    let total_steps = base_steps as i64 + q * cycle_len as i64;
                    if total_steps >= 1 {
                        let total_sum = base_sum + q * cycle_sum;
                        global_best = global_best.max(total_sum);
                    }
                } else {
                    // Best is with q = 0 (taking cycles doesn't help).
                    if base_steps >= 1 {
                        global_best = global_best.max(base_sum);
                    }
                }

                // Also consider the case where cycle_sum > 0 but taking one fewer cycle might allow
                // a different remainder to be better; since we iterate all remainders, this is covered.
            }

            // Additionally, if cycle_sum > 0, the best might be achieved by taking some steps into
            // the cycle after doing the maximum number of full cycles from cycle_start.
            // The remainder loop above already accounts for all r with q chosen as max_q for that r.
        }
    }

    println!("{}", global_best);
}
