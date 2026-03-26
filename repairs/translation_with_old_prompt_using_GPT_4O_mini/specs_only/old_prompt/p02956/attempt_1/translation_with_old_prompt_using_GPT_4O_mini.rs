use std::cmp::Ordering;

#[derive(Debug)]
struct Data {
    key: i32,
    id: i32,
}

fn compute_result(n: usize, mut d: Vec<Data>) -> i64 {
    // Sort the data pairs based on the key
    d.sort_by(|a, b| a.key.cmp(&b.key));

    // Initialize a Binary Indexed Tree (BIT) for counting occurrences of ids
    let mut bit = vec![0; n + 1]; // BIT size is N + 1 for 1-based indexing
    let mut result: i64 = 0;

    // Function to update the BIT
    fn update(bit: &mut Vec<i32>, index: usize, value: i32) {
        let mut idx = index as usize;
        while idx < bit.len() {
            bit[idx] += value;
            idx += idx & (!idx + 1); // Move to the next index
        }
    }

    // Function to query the BIT
    fn query(bit: &Vec<i32>, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as usize;
        while idx > 0 {
            sum += bit[idx];
            idx -= idx & (!idx + 1); // Move to the parent index
        }
        sum
    }

    // Process each data pair
    for i in 0..n {
        let id = d[i].id;
        // Update the BIT with the current id
        update(&mut bit, id as usize, 1);
        
        // Count how many ids are less than the current id
        let count = query(&bit, id as usize - 1);
        
        // Compute the result based on the count
        result += count as i64;
    }

    // Return the final computed result
    result
}

fn main() {
    // Example usage
    let n = 5;
    let d = vec![
        Data { key: 3, id: 1 },
        Data { key: 1, id: 3 },
        Data { key: 2, id: 2 },
        Data { key: 5, id: 5 },
        Data { key: 4, id: 4 },
    ];

    let result = compute_result(n, d);
    println!("{}", result);
}
