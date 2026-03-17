use std::collections::HashMap;
use std::io::{self, BufRead};

struct FlowNetwork {
    capacities: HashMap<(char, char), i32>,
    costs: HashMap<(char, char), i32>,
}

impl FlowNetwork {
    fn new() -> Self {
        FlowNetwork {
            capacities: HashMap::new(),
            costs: HashMap::new(),
        }
    }

    // This function would implement the min-cost flow algorithm.
    // For the sake of this translation, we will assume it returns a vector of characters.
    fn min_cost_flow(&self) -> Vec<char> {
        // Placeholder for the actual min-cost flow logic.
        // In a real implementation, this would compute the flow and return the decoded characters.
        vec![]
    }
}

fn decode_messages(n: usize, messages: Vec<String>) -> String {
    let mut flow_network = FlowNetwork::new();
    
    // Populate the flow network based on character mappings.
    for message in messages {
        for c in message.chars() {
            if c.is_ascii_alphabetic() {
                // Here we would set capacities and costs based on the character mappings.
                // For simplicity, we assume a direct mapping with a cost of 1.
                let cost = 1; // Example cost
                flow_network.capacities.insert((c, c), 1); // Capacity of 1 for self-loop
                flow_network.costs.insert((c, c), cost);
            }
        }
    }

    // Compute the min-cost flow and get the decoded characters.
    let decoded_chars = flow_network.min_cost_flow();

    // Separate uppercase and lowercase characters.
    let mut upper: Vec<char> = decoded_chars.iter().filter(|&&c| c.is_ascii_uppercase()).cloned().collect();
    let mut lower: Vec<char> = decoded_chars.iter().filter(|&&c| c.is_ascii_lowercase()).cloned().collect();

    // Sort characters to maintain a consistent output order.
    upper.sort();
    lower.sort();

    // Combine the results and return as a single string.
    upper.extend(lower);
    upper.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of lines (N).
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the N lines of messages.
    let messages: Vec<String> = lines.take(n).filter_map(Result::ok).collect();

    // Decode the messages and print the result.
    let result = decode_messages(n, messages);
    println!("{}", result);
}
