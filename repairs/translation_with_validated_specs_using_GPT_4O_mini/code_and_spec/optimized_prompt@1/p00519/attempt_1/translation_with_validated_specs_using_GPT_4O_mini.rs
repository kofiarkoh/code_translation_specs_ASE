use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Taxi {
    unit_cost: i32,
    limit: i32,
}

#[derive(Debug)]
struct Mati {
    taxi: Taxi,
    loads: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Status {
    town: usize,
    remaining_rides: usize,
}

impl Status {
    fn cost(&self, min_cost: &Vec<Vec<i32>>) -> i32 {
        min_cost[self.town][self.remaining_rides] - 1
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    if n == 0 || (k > 0 && k > (n * (n - 1)) / 2) {
        return; // Invalid input based on the specification
    }

    let mut taxis = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let unit_cost: i32 = parts.next().unwrap().parse().unwrap();
        let limit: i32 = parts.next().unwrap().parse().unwrap();
        taxis.push(Taxi { unit_cost, limit });
    }

    let mut mati = vec![Mati { taxi: Taxi { unit_cost: 0, limit: 0 }, loads: Vec::new() }; n];
    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        mati[a].loads.push(b);
        mati[b].loads.push(a);
    }

    let mut min_cost = vec![vec![0; 5001]; 5000];
    let mut in_heap = vec![vec![false; 5001]; 5000];
    let mut heap = BinaryHeap::new();
    let initial_status = Status { town: 0, remaining_rides: 0 };

    heap.push((0, initial_status)); // (cost, status)

    while let Some((cur_cost, cur_status)) = heap.pop() {
        let cur_cost = -cur_cost; // Convert back to positive cost

        if cur_status.town == n - 1 {
            println!("{}", cur_cost);
            return; // Destination reached
        }

        if cur_status.remaining_rides > 0 {
            for &next_town in &mati[cur_status.town].loads {
                let next_status = Status { town: next_town, remaining_rides: cur_status.remaining_rides - 1 };
                if min_cost[next_status.town][next_status.remaining_rides] == 0 || min_cost[next_status.town][next_status.remaining_rides] - 1 > cur_cost {
                    min_cost[next_status.town][next_status.remaining_rides] = cur_cost + 1;
                    if !in_heap[next_status.town][next_status.remaining_rides] {
                        in_heap[next_status.town][next_status.remaining_rides] = true;
                        heap.push((-cur_cost, next_status));
                    }
                }
            }
        }

        let next_status = Status { town: cur_status.town, remaining_rides: taxis[cur_status.town].limit as usize };
        if min_cost[next_status.town][next_status.remaining_rides] == 0 || min_cost[next_status.town][next_status.remaining_rides] - 1 > cur_cost + taxis[cur_status.town].unit_cost {
            min_cost[next_status.town][next_status.remaining_rides] = cur_cost + taxis[cur_status.town].unit_cost + 1;
            if !in_heap[next_status.town][next_status.remaining_rides] {
                in_heap[next_status.town][next_status.remaining_rides] = true;
                heap.push(-(cur_cost + taxis[cur_status.town].unit_cost), next_status);
            }
        }
    }
}
