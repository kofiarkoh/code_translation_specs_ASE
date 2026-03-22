use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

fn main() {
    // Whitespace-delimited token stream; line boundaries are irrelevant.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut outputs: Vec<String> = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more datasets.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // Termination condition: no further output.
        }

        let m: usize = it.next().unwrap().parse().unwrap();
        let u: i64 = it.next().unwrap().parse().unwrap();
        let cap: i64 = 10 * u;

        let origin_name = it.next().unwrap().to_string();
        let dest_name = it.next().unwrap().to_string();

        // Map city names to indices, created from origin/destination and road endpoints.
        let mut city_id: HashMap<String, usize> = HashMap::new();
        let mut next_id: usize = 0;

        let mut get_id = |name: &str, city_id: &mut HashMap<String, usize>, next_id: &mut usize| {
            if let Some(&id) = city_id.get(name) {
                id
            } else {
                let id = *next_id;
                *next_id += 1;
                city_id.insert(name.to_string(), id);
                id
            }
        };

        let origin = get_id(&origin_name, &mut city_id, &mut next_id);
        let dest = get_id(&dest_name, &mut city_id, &mut next_id);

        // Temporarily store edges until we know the final number of cities.
        let mut edges: Vec<(usize, usize, i64)> = Vec::with_capacity(n);
        for _ in 0..n {
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            let d: i64 = it.next().unwrap().parse().unwrap();
            let ia = get_id(a, &mut city_id, &mut next_id);
            let ib = get_id(b, &mut city_id, &mut next_id);
            edges.push((ia, ib, d));
        }

        let city_count = next_id;
        let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); city_count];
        for (a, b, d) in edges {
            adj[a].push((b, d));
            adj[b].push((a, d));
        }

        // Refueling cities: tokens that do not match any known city are ignored.
        let mut is_refuel = vec![false; city_count];
        for _ in 0..m {
            let name = it.next().unwrap();
            if let Some(&id) = city_id.get(name) {
                is_refuel[id] = true;
            }
        }

        // If origin == destination, minimum distance is 0.
        if origin == dest {
            outputs.push("0".to_string());
            continue;
        }

        // Dijkstra over state (city, remaining_range).
        // Remaining range is an integer in [0..cap] (as implied by the model).
        // If cap is negative (unspecified by spec), we keep behavior narrow: no travel possible.
        if cap < 0 {
            outputs.push("-1".to_string());
            continue;
        }
        let cap_usize = cap as usize;

        // dist[city][range] = min total distance to reach that exact state.
        // Use i64 for distances as per input model; behavior for overflow is unspecified.
        let inf: i64 = i64::MAX / 4;
        let mut dist: Vec<Vec<i64>> = vec![vec![inf; cap_usize + 1]; city_count];

        let start_range = cap_usize;
        dist[origin][start_range] = 0;

        let mut pq: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
        pq.push((Reverse(0), origin, start_range));

        let mut answer: Option<i64> = None;

        while let Some((Reverse(cur_d), city, range)) = pq.pop() {
            if cur_d != dist[city][range] {
                continue;
            }

            if city == dest {
                answer = Some(cur_d);
                break;
            }

            // Refueling occurs upon being at the city, before traversing the next road.
            let effective_range = if is_refuel[city] { cap_usize } else { range };

            for &(nxt, w) in &adj[city] {
                // Road distances are assumed nonnegative; if negative, behavior is unspecified.
                if w < 0 {
                    continue;
                }
                let w_usize = w as usize;
                if w_usize <= effective_range {
                    let new_range = effective_range - w_usize;
                    let new_d = cur_d + w;
                    if new_d < dist[nxt][new_range] {
                        dist[nxt][new_range] = new_d;
                        pq.push((Reverse(new_d), nxt, new_range));
                    }
                }
            }
        }

        outputs.push(match answer {
            Some(v) => v.to_string(),
            None => "-1".to_string(),
        });
    }

    print!("{}", outputs.join("\n"));
    if !outputs.is_empty() {
        print!("\n");
    }
}
