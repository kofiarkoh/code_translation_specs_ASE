use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Connection {
    city_a: usize,
    city_b: usize,
    cost: usize,
    time: usize,
}

#[derive(Debug)]
struct State {
    city: usize,
    time: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a min-heap based on travel time
        other.time.cmp(&self.time) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for State {}

fn dijkstra(num_cities: usize, connections: &[Connection], budget: usize) -> Result<Vec<usize>, &'static str> {
    let mut min_times = vec![usize::MAX; num_cities];
    min_times[0] = 0; // Starting city time is always zero

    let mut heap = BinaryHeap::new();
    heap.push(State { city: 0, time: 0 });

    while let Some(State { city, time }) = heap.pop() {
        if time > min_times[city] {
            continue; // Skip if we found a better time already
        }

        for connection in connections.iter() {
            let (next_city, cost, travel_time) = if connection.city_a == city {
                (connection.city_b, connection.cost, connection.time)
            } else if connection.city_b == city {
                (connection.city_a, connection.cost, connection.time)
            } else {
                continue; // Not a valid connection
            };

            if cost > budget {
                continue; // Skip if the cost exceeds the budget
            }

            let new_time = time + travel_time;
            if new_time < min_times[next_city] {
                min_times[next_city] = new_time;
                heap.push(State { city: next_city, time: new_time });
            }
        }
    }

    // Check for unreachable cities
    for &time in &min_times {
        if time == usize::MAX {
            return Err("Failed to compute a valid path");
        }
    }

    Ok(min_times)
}

fn main() {
    let num_cities = 5; // Example input
    let connections = vec![
        Connection { city_a: 0, city_b: 1, cost: 10, time: 2 },
        Connection { city_a: 1, city_b: 2, cost: 5, time: 1 },
        Connection { city_a: 0, city_b: 2, cost: 15, time: 5 },
        Connection { city_a: 1, city_b: 3, cost: 5, time: 2 },
        Connection { city_a: 2, city_b: 4, cost: 10, time: 3 },
        Connection { city_a: 3, city_b: 4, cost: 4, time: 1 },
        Connection { city_a: 0, city_b: 3, cost: 8, time: 4 }, // Added connection
        Connection { city_a: 1, city_b: 4, cost: 6, time: 3 }, // Added connection
    ];
    let budget = 20; // Example input

    match dijkstra(num_cities, &connections, budget) {
        Ok(min_times) => {
            for time in min_times {
                if time == usize::MAX {
                    println!("-1"); // Print -1 for unreachable cities
                } else {
                    println!("{}", time);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
