use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // The specification does not define the exact "pattern", but the sample indicates:
    // - The grid contains non-negative integers; cells with value > 0 are "points of interest".
    // - We start at (0,0) and must visit all points of interest.
    // - Movement is 4-directional with unit cost per step.
    // - The computed sum is the minimum total travel distance to visit all points of interest,
    //   where visiting a cell with value k counts as visiting k times (i.e., multiplicity).
    //
    // This matches the sample: total multiplicity is 12, and the minimum route length is 34.
    //
    // Implementation:
    // - Expand points by multiplicity into a list of targets.
    // - Precompute pairwise shortest path distances between start and all targets using BFS on open grid
    //   (which equals Manhattan distance, but we keep BFS to avoid assuming more than 4-neighborhood).
    // - Solve TSP path (start -> visit all targets in any order) with DP over subsets.
    //
    // Note: If multiplicities are large, this can blow up; the spec provides no constraints.
    // We keep the narrowest approach that matches observed behavior.

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut outputs = Vec::new();

    loop {
        let x: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let y: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        if x == 0 && y == 0 {
            break;
        }

        let mut grid = vec![vec![0i32; y]; x];
        for i in 0..x {
            for j in 0..y {
                grid[i][j] = it.next().unwrap().parse().unwrap();
            }
        }

        // Collect targets with multiplicity.
        let mut targets: Vec<(usize, usize)> = Vec::new();
        for i in 0..x {
            for j in 0..y {
                let k = grid[i][j];
                if k > 0 {
                    for _ in 0..k {
                        targets.push((i, j));
                    }
                }
            }
        }

        if targets.is_empty() {
            outputs.push("0".to_string());
            continue;
        }

        // Nodes: 0 = start, 1..=n = each target instance.
        let n = targets.len();
        let mut nodes = Vec::with_capacity(n + 1);
        nodes.push((0usize, 0usize));
        nodes.extend_from_slice(&targets);

        // Precompute distances between all nodes using BFS from each node.
        // Grid has no obstacles specified; still, BFS is correct for 4-neighborhood.
        let mut dist_mat = vec![vec![0i32; n + 1]; n + 1];
        for src in 0..=n {
            let (sx, sy) = nodes[src];
            let mut dist = vec![vec![-1i32; y]; x];
            let mut q = std::collections::VecDeque::new();
            dist[sx][sy] = 0;
            q.push_back((sx, sy));
            while let Some((i, j)) = q.pop_front() {
                let d = dist[i][j] + 1;
                if i > 0 && dist[i - 1][j] < 0 {
                    dist[i - 1][j] = d;
                    q.push_back((i - 1, j));
                }
                if i + 1 < x && dist[i + 1][j] < 0 {
                    dist[i + 1][j] = d;
                    q.push_back((i + 1, j));
                }
                if j > 0 && dist[i][j - 1] < 0 {
                    dist[i][j - 1] = d;
                    q.push_back((i, j - 1));
                }
                if j + 1 < y && dist[i][j + 1] < 0 {
                    dist[i][j + 1] = d;
                    q.push_back((i, j + 1));
                }
            }
            for dst in 0..=n {
                let (tx, ty) = nodes[dst];
                let d = dist[tx][ty];
                // Unreachable is unspecified; treat as very large to avoid panics.
                dist_mat[src][dst] = if d >= 0 { d } else { i32::MAX / 4 };
            }
        }

        // TSP DP: dp[mask][last] = min cost to start at 0, visit targets in mask, end at 'last' (1..=n).
        // mask is over n targets (bits 0..n-1 correspond to node index 1..=n).
        let full_mask = (1u64 << n) - 1;
        let inf = i32::MAX / 4;

        // Use Dijkstra over state space to avoid allocating 2^n * n if n is moderate but still large.
        // State: (mask, last_index) where last_index in 0..=n, with last_index=0 meaning at start with empty mask.
        // Goal: any state with mask==full_mask (end anywhere).
        let mut best = std::collections::HashMap::<(u64, usize), i32>::new();
        let mut heap = BinaryHeap::<(Reverse<i32>, u64, usize)>::new();

        best.insert((0, 0), 0);
        heap.push((Reverse(0), 0, 0));

        let mut answer = inf;

        while let Some((Reverse(cost), mask, last)) = heap.pop() {
            if let Some(&known) = best.get(&(mask, last)) {
                if cost != known {
                    continue;
                }
            } else {
                continue;
            }

            if mask == full_mask {
                if cost < answer {
                    answer = cost;
                }
                continue;
            }

            // Prune if already worse than best found.
            if cost >= answer {
                continue;
            }

            for t in 0..n {
                let bit = 1u64 << t;
                if (mask & bit) != 0 {
                    continue;
                }
                let next_mask = mask | bit;
                let next_node = t + 1; // node index in dist_mat
                let step = dist_mat[last][next_node];
                if step >= inf {
                    continue;
                }
                let next_cost = cost.saturating_add(step);
                let key = (next_mask, next_node);
                let update = match best.get(&key) {
                    Some(&v) => next_cost < v,
                    None => true,
                };
                if update {
                    best.insert(key, next_cost);
                    heap.push((Reverse(next_cost), next_mask, next_node));
                }
            }
        }

        outputs.push(answer.to_string());
    }

    if !outputs.is_empty() {
        print!("{}", outputs.join("\n"));
    }
}
