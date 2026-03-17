fn main() {
    // Read input values for height (h), width (w), and target (k)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse the input values as integers
    let h: i32 = iter.next().unwrap().parse().expect("Invalid input for height");
    let w: i32 = iter.next().unwrap().parse().expect("Invalid input for width");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input for target");

    // Iterate through all possible combinations of non-negative integers i and j
    for i in 0..=h {
        for j in 0..=w {
            // Calculate the expression (w*i) + (h*j) - 2*(i*j)
            let expression = (w * i) + (h * j) - 2 * (i * j);
            // Check if the expression equals k
            if expression == k {
                println!("Yes");
                return; // Exit early if condition is met
            }
        }
    }
    
    // If no combination satisfies the condition, print "No"
    println!("No");
}
