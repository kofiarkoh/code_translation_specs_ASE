use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Record {
    id: i32,
    attr1: i32,
    attr2: i32,
    attr3: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut values: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check the first integer for the number of records
        if values.is_empty() || values[0] == 0 {
            // Terminate if the first integer is zero or no records
            return;
        }

        let num_records = values[0] as usize;
        if num_records > 300 {
            // According to the specification, we should not exceed 300 records
            continue;
        }

        // Collect records
        let mut records: Vec<Record> = Vec::new();
        for i in 0..num_records {
            if values.len() < (i + 1) * 4 {
                // If there are not enough values for a complete record, skip
                break;
            }
            records.push(Record {
                id: values[(i * 4) + 1],
                attr1: values[(i * 4) + 2],
                attr2: values[(i * 4) + 3],
                attr3: values[(i * 4) + 4],
            });
        }

        // Sort records by attr1, attr2, and attr3
        records.sort_by(|a, b| {
            a.attr1.cmp(&b.attr1)
                .then_with(|| a.attr2.cmp(&b.attr2))
                .then_with(|| a.attr3.cmp(&b.attr3))
        });

        // Count frequencies of identifiers
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        for record in &records {
            *frequency.entry(record.id).or_insert(0) += 1;
        }

        // Collect identifiers based on frequency constraints
        let mut identifiers: Vec<i32> = frequency.keys().cloned().collect();
        identifiers.sort_by(|a, b| frequency[b].cmp(&frequency[a])); // Sort by frequency

        // Output up to 25 unique identifiers
        for id in identifiers.iter().take(25) {
            println!("{}", id);
        }
    }
}
