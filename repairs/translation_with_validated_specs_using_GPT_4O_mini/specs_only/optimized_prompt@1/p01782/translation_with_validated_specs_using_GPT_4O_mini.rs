use std::collections::HashMap;
use std::io::{self, BufRead};

struct FlowNetwork {
    flow: HashMap<char, usize>,
}

impl FlowNetwork {
    fn new() -> Self {
        FlowNetwork {
            flow: HashMap::new(),
        }
    }

    // This function simulates the flow aggregation based on input character frequencies.
    fn add_flow(&mut self, character: char) {
        *self.flow.entry(character).or_insert(0) += 1;
    }

    // Aggregate flow values for output
    fn aggregate_flow(&self) -> HashMap<char, usize> {
        self.flow.clone()
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of lines
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut flow_network = FlowNetwork::new();

    // Read each line and populate the flow network
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        for c in line.chars() {
            // Add flow for each character
            flow_network.add_flow(c);
        }
    }

    // Aggregate the flow values to produce the output
    let aggregated_flow = flow_network.aggregate_flow();

    // Construct the output string based on aggregated flow
    let mut output: Vec<char> = aggregated_flow.iter()
        .flat_map(|(&char, &count)| std::iter::repeat(char).take(count))
        .collect();

    // Sort the output to match expected order (uppercase before lowercase)
    output.sort_unstable_by(|a, b| {
        // Sort uppercase letters before lowercase letters
        if a.is_ascii_uppercase() && b.is_ascii_lowercase() {
            std::cmp::Ordering::Less
        } else if a.is_ascii_lowercase() && b.is_ascii_uppercase() {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    // Print the decoded string, only unique characters in the order of appearance
    let unique_output: String = output.iter().unique().collect();

    println!("{}", unique_output);
}
