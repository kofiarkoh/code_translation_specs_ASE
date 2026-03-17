fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read the values associated with each node
    let values: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the parent-child relationships
    let parents: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Ensure the number of nodes matches the input
    assert_eq!(values.len(), n);
    assert_eq!(parents.len(), n);

    // Create a vector to hold the cumulative values for each node
    let mut cumulative_values = vec![0; n];

    // Calculate the cumulative values based on parent-child relationships
    for i in 0..n {
        let mut current = i;
        let mut sum = 0;

        // Traverse up the graph to accumulate values
        while current != -1 {
            sum += values[current as usize];
            current = parents[current as usize];
        }

        cumulative_values[i] = sum;
    }

    // Calculate the final aggregate value
    let aggregate_value: i64 = cumulative_values.iter().sum();

    // Output the result
    println!("{}", aggregate_value);
}
