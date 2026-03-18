use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn min_cost_merge(sequence: Vec<i64>) -> i64 {
    // Use a min-heap to efficiently get the two smallest elements
    let mut heap = BinaryHeap::new();
    
    // Push all elements into the heap (inverted to create a min-heap)
    for &num in &sequence {
        heap.push(std::cmp::Reverse(num));
    }

    let mut total_cost = 0;

    // While there is more than one element in the heap
    while heap.len() > 1 {
        // Pop the two smallest elements
        let std::cmp::Reverse(first) = heap.pop().unwrap();
        let std::cmp::Reverse(second) = heap.pop().unwrap();

        // Calculate the cost of merging these two
        let merge_cost = first + second;
        total_cost += merge_cost;

        // Push the merged cost back into the heap
        heap.push(std::cmp::Reverse(merge_cost));
    }

    total_cost
}

fn main() {
    // Read the number of elements
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the sequence of integers
    let mut sequence = Vec::new();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    sequence.extend(input.trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()));

    // Ensure the input size matches the expected number of elements
    assert_eq!(sequence.len(), n);

    // Calculate and print the minimum cost of merging
    let result = min_cost_merge(sequence);
    println!("{}", result);
}
