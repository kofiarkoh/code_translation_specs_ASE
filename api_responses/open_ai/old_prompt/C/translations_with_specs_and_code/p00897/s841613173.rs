use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

const HASHSIZ: usize = 29989;
const MAX: usize = 6002;

#[derive(Debug)]
struct HashEntry {
    city: String,
    id: Option<usize>,
}

struct HashTable {
    table: Vec<HashEntry>,
}

impl HashTable {
    fn new() -> Self {
        let mut table = Vec::with_capacity(HASHSIZ);
        for _ in 0..HASHSIZ {
            table.push(HashEntry {
                city: String::new(),
                id: None,
            });
        }
        HashTable { table }
    }

    fn insert(&mut self, city: String, id: usize) -> Option<usize> {
        let mut hash = 0;
        for c in city.chars() {
            hash = (hash << 6) + c as usize;
        }
        let index = hash % HASHSIZ;

        let mut entry = &mut self.table[index];
        while entry.id.is_some() {
            if entry.city == city {
                return entry.id;
            }
            if entry.city.is_empty() {
                break;
            }
            entry = &mut self.table[(index + 1) % HASHSIZ];
        }
        entry.city = city;
        entry.id = Some(id);
        None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    city: usize,
    fuel: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    adj: Vec<Vec<(usize, usize)>>, // (neighbor, distance)
    lpg: Vec<bool>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); size],
            lpg: vec![false; size],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, distance: usize) {
        self.adj[from].push((to, distance));
        self.adj[to].push((from, distance)); // Undirected graph
    }

    fn set_lpg(&mut self, city: usize) {
        self.lpg[city] = true;
    }
}

fn dijkstra(graph: &Graph, cap: usize, start: usize, goal: usize) -> isize {
    let mut dist = vec![vec![usize::MAX; cap + 1]; graph.adj.len()];
    let mut heap = BinaryHeap::new();
    dist[start][cap] = 0;
    heap.push(State { cost: 0, city: start, fuel: cap });

    while let Some(State { cost, city, fuel }) = heap.pop() {
        if city == goal {
            return cost as isize;
        }
        if dist[city][fuel] < cost {
            continue;
        }

        let mut current_fuel = fuel;
        if graph.lpg[city] {
            current_fuel = cap;
        }

        for &(neighbor, distance) in &graph.adj[city] {
            let new_cost = cost + distance;
            let new_fuel = current_fuel.saturating_sub(distance);
            if new_fuel < dist[neighbor].len() && dist[neighbor][new_fuel] > new_cost {
                dist[neighbor][new_fuel] = new_cost;
                heap.push(State { cost: new_cost, city: neighbor, fuel: new_fuel });
            }
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut hash_table = HashTable::new();
    let mut graph = Graph::new(MAX);
    let mut memo = Vec::new();
    
    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        let m: usize = lines.next().unwrap().trim().parse().unwrap();
        let cap: usize = lines.next().unwrap().trim().parse::<usize>().unwrap() * 10;

        hash_table = HashTable::new();
        graph = Graph::new(MAX);
        memo.clear();

        let mut city_count = 0;

        for _ in 0..n {
            let city_a = lines.next().unwrap();
            let city_b = lines.next().unwrap();
            let distance: usize = lines.next().unwrap().trim().parse().unwrap();

            let id_a = hash_table.insert(city_a.clone(), city_count).unwrap_or_else(|| {
                city_count += 1;
                city_count - 1
            });
            let id_b = hash_table.insert(city_b.clone(), city_count).unwrap_or_else(|| {
                city_count += 1;
                city_count - 1
            });

            graph.add_edge(id_a, id_b, distance);
        }

        for _ in 0..m {
            let lpg_city = lines.next().unwrap();
            if let Some(id) = hash_table.insert(lpg_city.clone(), 0) {
                graph.set_lpg(id);
            }
        }

        let result = dijkstra(&graph, cap, 0, 1);
        println!("{}", result);
    }
}
