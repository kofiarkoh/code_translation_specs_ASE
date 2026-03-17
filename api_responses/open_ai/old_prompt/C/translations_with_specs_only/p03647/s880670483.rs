fn main() {
    // Read the number of iterations (n)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input for n");

    // If n is zero or negative, output "IMPOSSIBLE"
    if n <= 0 {
        println!("IMPOSSIBLE");
        return;
    }

    // Read pairs of integers (a, b) for n iterations
    for _ in 0..n {
        let mut pair_input = String::new();
        std::io::stdin().read_line(&mut pair_input).expect("Failed to read line");
        let mut iter = pair_input.trim().split_whitespace();
        
        // Parse a and b from the input
        let a: i32 = iter.next().expect("Missing a").parse().expect("Invalid input for a");
        let b: i32 = iter.next().expect("Missing b").parse().expect("Invalid input for b");

        // Here we would evaluate the condition based on a and b
        // Since the specification does not define the condition, we assume a placeholder
        // For demonstration, we will just print "POSSIBLE" for any valid (a, b)
        // In a real scenario, replace this with the actual condition check
        if a + b > 0 { // Placeholder condition
            println!("POSSIBLE");
        } else {
            println!("IMPOSSIBLE");
        }
    }
}
