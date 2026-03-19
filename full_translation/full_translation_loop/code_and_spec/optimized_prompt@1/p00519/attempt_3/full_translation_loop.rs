use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Status {
    town: usize,
    kaisu: usize,
}

#[derive(Debug, Clone)]
struct Taxi {
    fare: usize,
    limit: usize,
}

#[derive(Debug, Clone)]
struct Mati {
    taxi: Taxi,
    loads: Vec<usize>,
}

impl Mati {
    fn new() -> Self {
        Mati {
            taxi: Taxi { fare: 0, limit: 0 },
            loads: Vec::new(),
        }
    }

    fn add_load(&mut self, to: usize) {
        self.loads.push(to);
    }
}

fn read_cost(mincost: &Vec<Vec<isize>>, stat: Status) -> isize {
    mincost[stat.town][stat.kaisu] - 1
}

fn update_cost(mincost: &mut Vec<Vec<isize>>, stat: Status, cost: isize) {
    if mincost[stat.town][stat.kaisu] == 0 || mincost[stat.town][stat.kaisu] - 1 > cost {
        mincost[stat.town][stat.kaisu] = cost + 1;
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

    let mut taxi = vec![Taxi { fare: 0, limit: 0 }; n];
    let mut mati = vec![Mati::new(); n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        taxi[i].fare = parts.next().unwrap().parse().unwrap();
        taxi[i].limit = parts.next().unwrap().parse().unwrap();
        mati[i].taxi = taxi[i].clone();
    }

    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse().unwrap() - 1;
        mati[a].add_load(b);
        mati[b].add_load(a);
    }

    let mut heap = BinaryHeap::new();
    let mut mincost = vec![vec![0; 5001]; n];
    let mut in_heap = vec![vec![false; 5001]; n];

    let initial = Status { town: 0, kaisu: 0 };
    heap.push((0, initial)); // (cost, status)
    update_cost(&mut mincost, initial, 0);

    while let Some((cur_cost, cur_status)) = heap.pop() {
        let cur_cost = -cur_cost; // Invert cost for min-heap behavior
        if cur_status.town == n - 1 {
            println!("{}", cur_cost);
            return;
        }

        if cur_status.kaisu > 0 {
            let next_kaisu = cur_status.kaisu - 1;
            for &next_town in &mati[cur_status.town].loads {
                let next_status = Status { town: next_town, kaisu: next_kaisu };
                if !in_heap[next_status.town][next_status.kaisu] {
                    update_cost(&mut mincost, next_status, cur_cost);
                    heap.push((-cur_cost, next_status));
                    in_heap[next_status.town][next_status.kaisu] = true;
                }
            }
        }

        let next_status = Status {
            town: cur_status.town,
            kaisu: mati[cur_status.town].taxi.limit,
        };
        if !in_heap[next_status.town][next_status.kaisu] {
            update_cost(&mut mincost, next_status, cur_cost as isize + mati[cur_status.town].taxi.fare as isize);
            heap.push(
                (-(cur_cost as isize + mati[cur_status.town].taxi.fare as isize), next_status)
            );
            in_heap[next_status.town][next_status.kaisu] = true;
        }
    }
}
