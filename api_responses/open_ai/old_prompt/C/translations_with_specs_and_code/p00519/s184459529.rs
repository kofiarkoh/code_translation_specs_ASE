use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Taxi {
    untin: i32,
    limit: i32,
}

#[derive(Debug)]
struct Mati {
    taxi: Taxi,
    loads_num: usize,
    loads_cap: usize,
    loads: Vec<usize>,
}

impl Mati {
    fn new() -> Self {
        Mati {
            taxi: Taxi { untin: 0, limit: 0 },
            loads_num: 0,
            loads_cap: 0,
            loads: Vec::new(),
        }
    }

    fn add_load(&mut self, to: usize) {
        if self.loads_num >= self.loads_cap {
            self.loads_cap = if self.loads_cap == 0 { 1 } else { self.loads_cap * 2 };
            self.loads.resize(self.loads_cap, 0);
        }
        self.loads[self.loads_num] = to;
        self.loads_num += 1;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Status {
    town: usize,
    kaisu: usize,
}

impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kaisu.cmp(&other.kaisu).then_with(|| self.town.cmp(&other.town))
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let k: usize = params.next().unwrap().parse().unwrap();

    let mut taxi = vec![Taxi::default(); n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut taxi_data = line.split_whitespace();
        taxi[i].untin = taxi_data.next().unwrap().parse().unwrap();
        taxi[i].limit = taxi_data.next().unwrap().parse().unwrap();
    }

    let mut mati = vec![Mati::new(); n];
    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut connection = line.split_whitespace();
        let a: usize = connection.next().unwrap().parse().unwrap() - 1;
        let b: usize = connection.next().unwrap().parse().unwrap() - 1;
        mati[a].add_load(b);
        mati[b].add_load(a);
    }

    let mut heap = BinaryHeap::new();
    let initial = Status { town: 0, kaisu: 0 };
    let mut status_mincost = vec![vec![0; 5001]; 5000];
    let mut in_heap = vec![vec![false; 5001]; 5000];

    heap.push(initial.clone());
    status_mincost[0][0] = 0;

    while let Some(cur_status) = heap.pop() {
        let cur_cost = status_mincost[cur_status.town][cur_status.kaisu];

        if cur_status.town == n - 1 {
            println!("{}", cur_cost);
            return;
        }

        if cur_status.kaisu > 0 {
            let next_kaisu = cur_status.kaisu - 1;
            for &next_town in &mati[cur_status.town].loads[..cur_status.loads_num] {
                let next_status = Status { town: next_town, kaisu: next_kaisu };
                insert_heap(&mut heap, &mut status_mincost, &mut in_heap, next_status, cur_cost);
            }
        }

        let next_status = Status { town: cur_status.town, kaisu: taxi[cur_status.town].limit };
        insert_heap(&mut heap, &mut status_mincost, &mut in_heap, next_status, cur_cost + taxi[cur_status.town].untin);
    }
}

fn insert_heap(heap: &mut BinaryHeap<Status>, status_mincost: &mut Vec<Vec<i32>>, in_heap: &mut Vec<Vec<bool>>, stat: Status, cost: i32) {
    let prev_cost = status_mincost[stat.town][stat.kaisu];
    if prev_cost == 0 || prev_cost - 1 > cost {
        status_mincost[stat.town][stat.kaisu] = cost + 1;
        if !in_heap[stat.town][stat.kaisu] {
            in_heap[stat.town][stat.kaisu] = true;
            heap.push(stat);
        }
    }
}
