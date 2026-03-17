fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of elements in the array
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the elements of the array
    let elements: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    a.extend(elements);

    // Read the number of queries
    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..q {
        // Read each query: start index, number of positions to shift, end index
        let query: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let b = query[0];
        let m = query[1];
        let e = query[2];

        // Perform the circular shift on the specified subarray
        let e_adjusted = e - b; // Adjust end index to be relative to the start index
        let m_adjusted = m % e_adjusted; // Normalize the shift amount

        // Create a temporary vector to hold the subarray
        let mut tmp: Vec<i32> = a[b..e].to_vec();

        // Apply the circular shift
        for i in 0..e_adjusted {
            a[b + i] = tmp[(i + e_adjusted - m_adjusted) % e_adjusted];
        }
    }

    // Output the modified array
    for (i, &value) in a.iter().enumerate() {
        if i == n - 1 {
            print!("{}", value); // No space after the last element
        } else {
            print!("{} ", value);
        }
    }
    println!(); // Ensure a newline at the end
}
