use std::collections::HashMap;
use std::io::{self, BufRead};

struct FlowNetwork {
    capacities: HashMap<(usize, usize), i32>,
    costs: HashMap<(usize, usize), i32>,
    flow: HashMap<(usize, usize), i32>,
    source: usize,
    sink: usize,
}

impl FlowNetwork {
    fn new(size: usize) -> Self {
        let mut capacities = HashMap::new();
        let mut costs = HashMap::new();
        let mut flow = HashMap::new();

        // Initialize the flow network with zero capacities and costs
        for i in 0..size {
            for j in 0..size {
                capacities.insert((i, j), 0);
                costs.insert((i, j), 0);
                flow.insert((i, j), 0);
            }
        }

        FlowNetwork {
            capacities,
            costs,
            flow,
            source: 0,
            sink: size - 1,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i32, cost: i32) {
        self.capacities.insert((from, to), capacity);
        self.costs.insert((from, to), cost);
    }

    fn min_cost_flow(&mut self) -> Vec<char> {
        // Placeholder for the min-cost flow algorithm
        // This should implement the actual logic to compute the flow
        // For now, we return an empty vector
        vec![]
    }
}

fn decode_messages(n: usize, mappings: Vec<Vec<char>>) -> Vec<char> {
    let mut flow_network = FlowNetwork::new(n);

    // Populate the flow network based on character mappings
    for i in 0..n {
        for j in 0..n {
            let cost = (mappings[i][j] as i32) - ('A' as i32); // Example cost based on character
            flow_network.add_edge(i, j, 1, cost); // Assuming capacity of 1 for each edge
        }
    }

    flow_network.min_cost_flow()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of lines
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the character mappings
    let mut mappings = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let chars: Vec<char> = line.chars().collect();
        mappings.push(chars);
    }

    // Decode messages
    let decoded_messages = decode_messages(n, mappings);

    // Print the decoded messages
    for ch in decoded_messages {
        print!("{}", ch);
    }
}
