use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy, Debug)]
struct Item {
    value: usize,
    energy_cost: usize,
    recovery: usize,
}

fn main() {
    // Input parsing: if incorrect/incomplete, exit non-zero.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let target: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };
    let n: usize = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let value: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let energy_cost: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        let recovery: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        items.push(Item {
            value,
            energy_cost,
            recovery,
        });
    }

    // Narrowest reasonable interpretation given the spec:
    // - We need minimum number of items to reach at least `target` total value.
    // - There is an "energy level" state; using an item requires paying energy_cost,
    //   then increases energy by recovery (bounded).
    // - Since no initial energy is specified, assume initial energy = 0.
    // - Since no maximum energy is specified, bound energy by a derived limit to keep memory bounded:
    //   max_energy = max energy_cost among items (if any), because having more energy than the
    //   maximum cost does not enable additional actions beyond being able to pay any cost.
    //   (This is a conservative, minimal bound consistent with "bounded memory space".)
    //
    // If target == 0, minimum items is 0.
    if target == 0 {
        println!("0");
        return;
    }

    let max_cost = items.iter().map(|x| x.energy_cost).max().unwrap_or(0);
    let max_energy = max_cost;

    // Memoization structure: dist[value][energy] = min items used to reach this state.
    // Value is capped at target (treat any >= target as target).
    const INF: u32 = u32::MAX / 4;
    let mut dist = vec![vec![INF; max_energy + 1]; target + 1];

    // BFS over uniform-cost edges (each item use costs 1 item), so BFS yields minimum items.
    let mut q = VecDeque::new();
    dist[0][0] = 0;
    q.push_back((0usize, 0usize));

    while let Some((v, e)) = q.pop_front() {
        let cur = dist[v][e];
        if v == target {
            // First time reaching target in BFS is minimal for that energy, but not necessarily
            // minimal across energies; still, BFS ensures nondecreasing item count, so we can
            // early-exit if this is the global minimum. To keep semantics simple, we don't early-exit.
        }

        for item in &items {
            if e < item.energy_cost {
                continue;
            }
            let mut nv = v.saturating_add(item.value);
            if nv > target {
                nv = target;
            }
            // Pay cost then recover; cap energy to max_energy.
            let ne = min(max_energy, e - item.energy_cost + item.recovery);

            let nd = cur.saturating_add(1);
            if nd < dist[nv][ne] {
                dist[nv][ne] = nd;
                q.push_back((nv, ne));
            }
        }
    }

    let ans = dist[target].iter().copied().min().unwrap_or(INF);
    if ans >= INF {
        // Specific message is not defined; choose the narrowest conventional indication.
        println!("IMPOSSIBLE");
    } else {
        println!("{}", ans);
    }
}
