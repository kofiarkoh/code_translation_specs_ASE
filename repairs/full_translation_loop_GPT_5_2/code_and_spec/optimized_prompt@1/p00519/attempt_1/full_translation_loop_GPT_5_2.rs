use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Taxi {
    fare: i32,
    limit: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    town: usize,
    kaisu: usize, // remaining rides
}

fn main() {
    // Read all input; if format is incorrect, terminate with error (non-zero).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match (it.next(), it.next()) {
        (Some(a), Some(b)) => {
            let n = a.parse::<usize>().ok();
            let k = b.parse::<usize>().ok();
            if n.is_none() || k.is_none() {
                std::process::exit(1);
            }
            // store k later
            (n.unwrap(), k.unwrap()).0
        }
        _ => std::process::exit(1),
    };
    // Re-parse to also get k (kept separate to preserve the same error behavior).
    let mut it2 = s.split_whitespace();
    let _n2: usize = it2.next().unwrap().parse().unwrap();
    let k: usize = it2.next().unwrap().parse().unwrap();

    // Read taxis
    let mut taxi: Vec<Taxi> = Vec::with_capacity(n);
    for _ in 0..n {
        let fare = match it2.next() {
            Some(x) => x.parse::<i32>().ok().unwrap_or_else(|| std::process::exit(1)),
            None => std::process::exit(1),
        };
        let limit_i32 = match it2.next() {
            Some(x) => x.parse::<i32>().ok().unwrap_or_else(|| std::process::exit(1)),
            None => std::process::exit(1),
        };
        // Spec says non-negative; C code assumes it. If negative appears, treat as format error.
        if fare < 0 || limit_i32 < 0 {
            std::process::exit(1);
        }
        taxi.push(Taxi {
            fare,
            limit: limit_i32 as usize,
        });
    }

    // Build adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..k {
        let a = match it2.next() {
            Some(x) => x.parse::<i32>().ok().unwrap_or_else(|| std::process::exit(1)),
            None => std::process::exit(1),
        };
        let b = match it2.next() {
            Some(x) => x.parse::<i32>().ok().unwrap_or_else(|| std::process::exit(1)),
            None => std::process::exit(1),
        };
        // C code decrements; input is 1-based.
        let (mut a, mut b) = (a - 1, b - 1);
        if a < 0 || b < 0 || a as usize >= n || b as usize >= n {
            std::process::exit(1);
        }
        let (a, b) = (a as usize, b as usize);
        adj[a].push(b);
        adj[b].push(a);
    }

    // Determine maximum limit to size DP arrays.
    // C code uses fixed [5000][5001]; we size to max_limit+1 to preserve semantics.
    let max_limit = taxi.iter().map(|t| t.limit).max().unwrap_or(0);

    // status_mincost[town][kaisu] stores cost+1, with 0 meaning "unset" (as in C).
    let mut mincost: Vec<Vec<i32>> = vec![vec![0; max_limit + 1]; n];
    let mut in_heap: Vec<Vec<bool>> = vec![vec![false; max_limit + 1]; n];

    let read_cost = |mincost: &Vec<Vec<i32>>, st: State| -> i32 { mincost[st.town][st.kaisu] - 1 };

    let mut heap: BinaryHeap<(Reverse<i32>, State)> = BinaryHeap::new();

    let mut update_cost = |mincost: &mut Vec<Vec<i32>>, st: State, cost: i32| {
        let cell = &mut mincost[st.town][st.kaisu];
        if *cell == 0 || *cell - 1 > cost {
            *cell = cost + 1;
        }
    };

    let mut insert = |heap: &mut BinaryHeap<(Reverse<i32>, State)>,
                      mincost: &mut Vec<Vec<i32>>,
                      in_heap: &mut Vec<Vec<bool>>,
                      st: State,
                      cost: i32| {
        let prev = read_cost(mincost, st);
        if prev < 0 || cost < prev {
            update_cost(mincost, st, cost);
            if !in_heap[st.town][st.kaisu] {
                in_heap[st.town][st.kaisu] = true;
                heap.push((Reverse(cost), st));
            }
        }
    };

    // Initial state: town 0, kaisu 0, cost 0
    insert(
        &mut heap,
        &mut mincost,
        &mut in_heap,
        State { town: 0, kaisu: 0 },
        0,
    );

    while let Some((Reverse(_prio_cost), st)) = heap.pop() {
        // Mirror C behavior: mark as not in heap when popped.
        in_heap[st.town][st.kaisu] = false;

        let cur_cost = read_cost(&mincost, st);

        if st.town == n - 1 {
            // Destination reached: print minimum cost and terminate.
            // If unreachable, produce no output (as per spec and C behavior).
            print!("{cur_cost}\n");
            break;
        }

        // Move to adjacent towns if we have remaining rides.
        if st.kaisu > 0 {
            let next_kaisu = st.kaisu - 1;
            for &to in &adj[st.town] {
                insert(
                    &mut heap,
                    &mut mincost,
                    &mut in_heap,
                    State {
                        town: to,
                        kaisu: next_kaisu,
                    },
                    cur_cost,
                );
            }
        }

        // Pay and (re)board taxi at current town: set remaining rides to this town's limit.
        // Note: This can be 0; then it just changes cost and keeps kaisu=0.
        let lim = taxi[st.town].limit;
        if lim <= max_limit {
            insert(
                &mut heap,
                &mut mincost,
                &mut in_heap,
                State {
                    town: st.town,
                    kaisu: lim,
                },
                cur_cost + taxi[st.town].fare,
            );
        }
    }
}
