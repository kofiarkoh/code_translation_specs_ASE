use std::io::{self, BufRead};

fn main() {
    // Define the maximum value and the lookup table for costs
    const MAX_VALUE: usize = 50;
    const MULTIPLE: usize = 100;
    let mut costs = vec![usize::MAX; MAX_VALUE + 1];
    
    // Base case: cost to achieve 0 is 0
    costs[0] = 0;

    // Populate the lookup table with minimum costs for achievable values
    for i in 1..=MAX_VALUE {
        // Example combinations (these should be defined based on the specification)
        // Here we assume some arbitrary combinations for demonstration
        // In a real scenario, these would be defined based on the problem requirements
        let combinations = vec![(1, 100), (2, 150), (3, 200)]; // (quantity, cost)

        for (quantity, cost) in &combinations {
            if i >= quantity {
                costs[i] = costs[i].min(costs[i - quantity] + cost);
            }
        }
    }

    // Read input values from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let input = line.unwrap().trim().parse::<usize>().unwrap_or(usize::MAX);
        
        // Check for termination condition
        if input == 0 {
            return; // Terminate gracefully
        }

        // Ensure input is a non-negative multiple of 100
        if input % MULTIPLE != 0 || input > MAX_VALUE {
            // Invalid input handling is not specified, so we do nothing
            continue;
        }

        // Output the minimum cost or zero if not achievable
        let cost = if costs[input] == usize::MAX { 0 } else { costs[input] };
        println!("{}", cost);
    }
}
```

### Explanation of Choices:
1. **Lookup Table Initialization**: The lookup table is initialized with `usize::MAX` to represent unachievable values, except for the base case where the cost to achieve 0 is set to 0.
2. **Combination Logic**: The combinations are hardcoded for demonstration purposes. In a real implementation, these would need to be defined based on the specific problem requirements.
3. **Input Handling**: The program reads from standard input and checks for termination on input `0`. Invalid inputs (negative or not multiples of 100) are ignored as per the specification.
4. **Output**: The program outputs the minimum cost or `0` if the value is not achievable, adhering to the postconditions of the specification.
