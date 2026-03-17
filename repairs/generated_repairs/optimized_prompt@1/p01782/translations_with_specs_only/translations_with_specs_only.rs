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

    fn min_cost_flow(&self, messages: &[String]) -> Vec<char> {
        let mut char_count = HashMap::new();

        for message in messages {
            for c in message.chars() {
                if c.is_ascii_alphabetic() {
                    *char_count.entry(c.to_ascii_uppercase()).or_insert(0) += 1;
                }
            }
        }

        let mut decoded_chars: Vec<char> = char_count.keys().cloned().collect();
        decoded_chars.sort();
        decoded_chars
    }
}

fn decode_messages(n: usize, messages: Vec<String>) -> String {
    let flow_network = FlowNetwork::new();
    let decoded_chars = flow_network.min_cost_flow(&messages);

    decoded_chars.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let messages: Vec<String> = lines.take(n).filter_map(Result::ok).collect();

    let result = decode_messages(n, messages);
    println!("{}", result);
}
