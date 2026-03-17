fn main() {
    // Read inputs h, w, and k from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();
    
    // Parse h, w, and k as non-negative integers
    let h: u32 = inputs.next().unwrap().parse().expect("Invalid input for h");
    let w: u32 = inputs.next().unwrap().parse().expect("Invalid input for w");
    let k: i32 = inputs.next().unwrap().parse().expect("Invalid input for k");

    // Iterate over all non-negative integers i and j within the bounds of h and w
    for i in 0..=h {
        for j in 0..=w {
            // Calculate the expression (w * i) + (h * j) - (2 * i * j)
            let expression = (w as i32 * i as i32) + (h as i32 * j as i32) - (2 * i as i32 * j as i32);
            
            // Check if the expression equals k
            if expression == k {
                println!("Yes");
                return; // Exit early if a solution is found
            }
        }
    }
    
    // If no valid (i, j) pair was found, output "No"
    println!("No");
}
