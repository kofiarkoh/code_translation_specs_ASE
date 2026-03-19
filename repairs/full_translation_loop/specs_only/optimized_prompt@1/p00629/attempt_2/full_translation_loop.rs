use std::io::{self, BufRead};
use std::collections::HashMap;

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
            Some(n) if n > 0 && n <= 1000 => n,
            _ => continue, // If zero or invalid, skip processing
        };

        let mut records: Vec<Record> = Vec::new();

        // Read the records
        for _ in 0..num_records {
            let record_line = reader.lines().next().expect("Failed to read record line").expect("Invalid line");
            let mut values = record_line.split_whitespace();
            let id: i32 = values.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let value1: i32 = values.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let value2: i32 = values.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let value3: i32 = values.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            records.push(Record { id, value1, value2, value3 });
        }

        // Sort records based on the defined criteria (e.g., by value1, then value2, then value3)
        records.sort_by(|a, b| {
            a.value1.cmp(&b.value1)
                .then_with(|| a.value2.cmp(&b.value2))
                .then_with(|| a.value3.cmp(&b.value3))
        });

        // Count frequencies of identifiers
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        for record in &records {
            *frequency.entry(record.id).or_insert(0) += 1;
        }

        // Collect identifiers based on frequency constraints
        let mut identifiers: Vec<i32> = frequency.keys().cloned().collect();
        identifiers.sort_by(|a, b| frequency[b].cmp(&frequency[a])); // Sort by frequency

        // Output up to 25 identifiers
        for id in identifiers.iter().take(25) {
            println!("{}", id);
        }
    }
}
