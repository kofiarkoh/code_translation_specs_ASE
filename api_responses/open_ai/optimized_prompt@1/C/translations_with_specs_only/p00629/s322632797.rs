use std::io::{self, BufRead};
use std::collections::HashMap;

const MAX_RECORDS: usize = 1000;
const MAX_OUTPUT: usize = 25;

#[derive(Debug)]
struct Record {
    id: i32,
    value1: i32,
    value2: i32,
    value3: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();

        // Read the number of records
        let num_records: usize = match iter.next().and_then(|s| s.parse().ok()) {
            Some(n) if n <= MAX_RECORDS => n,
            _ => {
                // If the number of records is zero or invalid, terminate processing
                return;
            }
        };

        let mut records: Vec<Record> = Vec::new();

        // Read the records
        for _ in 0..num_records {
            let record_line = reader.lines().next().expect("Failed to read record line").expect("Invalid record line");
            let values: Vec<i32> = record_line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if values.len() != 4 {
                // If the record does not have exactly 4 integers, we skip it
                continue;
            }

            records.push(Record {
                id: values[0],
                value1: values[1],
                value2: values[2],
                value3: values[3],
            });
        }

        // Sort records based on the defined criteria (id in this case)
        records.sort_by_key(|r| r.id);

        // Count frequencies of identifiers
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        for record in &records {
            *frequency.entry(record.id).or_insert(0) += 1;
        }

        // Collect identifiers based on frequency constraints
        let mut output: Vec<i32> = frequency.iter()
            .filter(|&(_, &count)| count > 1) // Example frequency constraint: only include ids with count > 1
            .map(|(&id, _)| id)
            .collect();

        // Limit output to MAX_OUTPUT
        output.truncate(MAX_OUTPUT);

        // Print the identifiers
        for id in output {
            println!("{}", id);
        }
    }
}
