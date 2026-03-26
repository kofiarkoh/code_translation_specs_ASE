use std::collections::BinaryHeap;
use std::cmp::Ordering;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Status {
    town: usize,
    kaisu: usize,
}

#[derive(Debug)]
struct HeapNode {
    cost: i32,
    status: Status,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Pathfinding {
    heap: BinaryHeap<HeapNode>,
    min_cost: Vec<Vec<i32>>,
    in_heap: Vec<Vec<bool>>,
    mati: Vec<Mati>,
    n: usize,
}

impl Pathfinding {
    fn new(n: usize) -> Self {
        Pathfinding {
            heap: BinaryHeap::new(),
            min_cost: vec![vec![i32::MAX; 5001]; n],
            in_heap: vec![vec![false; 5001]; n],
            mati: vec![Mati::new(); n],
            n,
        }
    }

    fn read_cost(&self, stat: Status) -> i32 {
        self.min_cost[stat.town][stat.kaisu] - 1
    }

    fn update_cost(&mut self, stat: Status, cost: i32) {
        if self.min_cost[stat.town][stat.kaisu] == 0 || self.min_cost[stat.town][stat.kaisu] - 1 > cost {
            self.min_cost[stat.town][stat.kaisu] = cost + 1;
        }
    }

    fn insert_heap(&mut self, stat: Status, cost: i32) {
        let prev_cost = self.read_cost(stat);
        if prev_cost < 0 || cost < prev_cost {
            self.update_cost(stat, cost);
            if !self.in_heap[stat.town][stat.kaisu] {
                self.in_heap[stat.town][stat.kaisu] = true;
                self.heap.push(HeapNode { cost, status: stat });
            }
        }
    }

    fn get_heap(&mut self) -> Option<Status> {
        if let Some(node) = self.heap.pop() {
            let ret = node.status;
            self.in_heap[ret.town][ret.kaisu] = false;
            Some(ret)
        } else {
            None
        }
    }

    fn run(&mut self) -> Option<i32> {
        let initial = Status { town: 0, kaisu: 0 };
        self.insert_heap(initial, 0);

        while let Some(cur_status) = self.get_heap() {
            let cur_cost = self.read_cost(cur_status);
            if cur_status.town == self.n - 1 {
                return Some(cur_cost);
            }

            if cur_status.kaisu > 0 {
                let next_kaisu = cur_status.kaisu - 1;
                for &next_town in &self.mati[cur_status.town].loads[..self.mati[cur_status.town].loads_num] {
                    self.insert_heap(Status { town: next_town, kaisu: next_kaisu }, cur_cost);
                }
            }

            let next_status = Status {
                town: cur_status.town,
                kaisu: self.mati[cur_status.town].taxi.limit,
            };
            self.insert_heap(next_status, cur_cost + self.mati[cur_status.town].taxi.untin);
        }
        None
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

    let mut pathfinding = Pathfinding::new(n);

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        pathfinding.mati[i].taxi.untin = parts.next().unwrap().parse().unwrap();
        pathfinding.mati[i].taxi.limit = parts.next().unwrap().parse().unwrap();
    }

    for _ in 0..k {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap() - 1;
        let b: usize = parts.next().unwrap().parse().unwrap() - 1;
        pathfinding.mati[a].add_load(b);
        pathfinding.mati[b].add_load(a);
    }

    if let Some(answer) = pathfinding.run() {
        println!("{}", answer);
    }
}
