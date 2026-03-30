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
        let mut table = vec![HashEntry { city: String::new(), id: None }; HASHSIZ + 2];
        HashTable { table }
    }

    fn insert(&mut self, city: String, id: usize) -> Option<usize> {
        let mut hash_value = 0;
        for byte in city.bytes() {
            hash_value = (hash_value << 6) + byte as usize;
        }
        let mut index = hash_value % HASHSIZ;

        while let Some(existing_id) = self.table[index].id {
            if self.table[index].city == city {
                return Some(existing_id);
            }
            index = (index + 1) % HASHSIZ;
        }
        self.table[index] = HashEntry { city, id: Some(id) };
        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueEntry {
    distance: usize,
    city: usize,
    fuel: usize,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse ordering for min-heap
    }
}

impl PartialOrd for QueueEntry {
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

    fn dijkstra(&self, start: usize, goal: usize, cap: usize) -> isize {
        let mut dist = vec![vec![usize::MAX; cap + 1]; self.adj.len()];
        let mut heap = BinaryHeap::new();
        dist[start][cap] = 0;
        heap.push(QueueEntry { distance: 0, city: start, fuel: cap });

        while let Some(QueueEntry { distance, city, fuel }) = heap.pop() {
            if city == goal {
                return distance as isize;
            }
            if dist[city][fuel] < distance {
                continue;
            }

            let mut current_fuel = if self.lpg[city] { cap } else { fuel };
            for &(neighbor, road_distance) in &self.adj[city] {
                let new_distance = distance + road_distance;
                let new_fuel = current_fuel.saturating_sub(road_distance);
                if new_fuel >= 0 && dist[neighbor][new_fuel] > new_distance {
                    dist[neighbor][new_fuel] = new_distance;
                    heap.push(QueueEntry { distance: new_distance, city: neighbor, fuel: new_fuel });
                }
            }
        }
        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);

    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        let m: usize = lines.next().unwrap().trim().parse().unwrap();
        let cap: usize = 10 * lines.next().unwrap().trim().parse::<usize>().unwrap();

        let mut hash_table = HashTable::new();
        let mut graph = Graph::new(n + 2);
        let mut city_count = 0;

        let mut city_names = vec!["".to_string(); n + 2];

        for i in 0..2 {
            let city_name = lines.next().unwrap();
            city_names[i] = city_name.clone();
            if hash_table.insert(city_name, i).is_none() {
                city_count += 1;
            }
        }

        for _ in 0..n {
            let city_a = lines.next().unwrap();
            let city_b = lines.next().unwrap();
            let distance: usize = lines.next().unwrap().trim().parse().unwrap();

            let id_a = hash_table.insert(city_a.clone(), city_count);
            if id_a.is_none() {
                city_names[city_count] = city_a;
                city_count += 1;
            }

            let id_b = hash_table.insert(city_b.clone(), city_count);
            if id_b.is_none() {
                city_names[city_count] = city_b;
                city_count += 1;
            }

            graph.add_edge(id_a.unwrap_or(city_count - 1), id_b.unwrap_or(city_count - 1), distance);
        }

        for _ in 0..m {
            let lpg_city = lines.next().unwrap();
            if let Some(id) = hash_table.insert(lpg_city, 0) {
                graph.set_lpg(id);
            }
        }

        let result = graph.dijkstra(0, 1, cap);
        println!("{}", result);
    }
}
