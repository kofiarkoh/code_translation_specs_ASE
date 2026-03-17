use std::io;

fn main() {
    // Read two integers from user input
    let (a, b) = read_input();

    // Calculate and print the smallest multiple of `a` that is also a multiple of `b`
    if b > 0 {
        let result = find_lcm(a, b);
        println!("{}", result);
    } else {
        // If b is 0, we enter an infinite loop as per the specification
        loop {}
    }

    // Return 0 upon successful execution
    std::process::exit(0);
}

// Function to read input from the user
fn read_input() -> (u64, u64) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input and parse the two integers
    let mut iter = input.split_whitespace();
    let a: u64 = iter.next().unwrap().parse().expect("Invalid input for a");
    let b: u64 = iter.next().unwrap().parse().expect("Invalid input for b");
    
    (a, b)
}

// Function to find the least common multiple (LCM) of a and b
fn find_lcm(a: u64, b: u64) -> u64 {
    let mut multiple = a;
    while multiple % b != 0 {
        multiple += a; // Increment by a to find the next multiple
    }
    multiple
}
