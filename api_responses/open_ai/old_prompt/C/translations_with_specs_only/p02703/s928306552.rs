use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const MAX_CITIES: usize = 50;
const MAX_CONNECTIONS: usize = 49;

#[derive(Debug)]
struct Connection {
    city: usize,
    cost: i32,
    time: i64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    
    let n: usize = params.next().unwrap().parse().unwrap(); // Number of cities
    let m: usize = params.next().unwrap().parse().unwrap(); // Number of train connections
    let s: i32 = params.next().unwrap().parse().unwrap(); // Initial budget

    let mut connections: Vec<Vec<Connection>> = vec![Vec::new(); n];

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut conn_params = line.split_whitespace();
        let u: usize = conn_params.next().unwrap().parse().unwrap(); // City u
        let v: usize = conn_params.next().unwrap().parse().unwrap(); // City v
        let a: i32 = conn_params.next().unwrap().parse().unwrap(); // Cost
        let b: i64 = conn_params.next().unwrap().parse().unwrap(); // Travel time

        connections[u].push(Connection { city: v, cost: a, time: b });
        connections[v].push(Connection { city: u, cost: a, time: b }); // Assuming bidirectional
    }

    let result = dijkstra(n, m, s, &connections);
    if result == 0 {
        return;
    }
}

fn dijkstra(n: usize, m: usize, s: i32, connections: &Vec<Vec<Connection>>) -> i32 {
    let max_budget = s as usize + 1; // Budget range from 0 to S
    let mut ans = vec![vec![i64::MAX; max_budget]; n]; // Minimum travel time
    let mut used = vec![vec![false; max_budget]; n]; // Used budget states

    let mut heap = BinaryHeap::new();
    ans[0][0] = 0; // Starting city with 0 time
    heap.push((0, 0, 0)); // (time, city, cost)

    while let Some((time, city, cost)) = heap.pop() {
        let time = -time; // Convert back to positive time
        if used[city][cost] {
            continue; // Already processed this state
        }
        used[city][cost] = true;

        for conn in &connections[city] {
            let new_cost = cost + conn.cost as usize;
            if new_cost < max_budget {
                let new_time = time + conn.time;
                if new_time < ans[conn.city][new_cost] {
                    ans[conn.city][new_cost] = new_time;
                    heap.push((-new_time, conn.city, new_cost));
                }
            }
        }
    }

    // Output the minimum travel time to each city
    for i in 0..n {
        let min_time = ans[i].iter().cloned().min().unwrap_or(i64::MAX);
        if min_time == i64::MAX {
            println!("-1"); // Indicating unreachable city
        } else {
            println!("{}", min_time);
        }
    }

    0 // Successful completion
}
