use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Check for the precondition that K should not be zero to avoid division by zero
    if k == 0 {
        // Undefined behavior as per the specification; we could handle this gracefully if needed
        // For now, we will just return without outputting anything
        return;
    }

    let groups = n / k;
    let result = if k % 2 == 1 {
        groups * groups * groups
    } else {
        let adjusted_groups = (n + k / 2) / k;
        groups * groups * groups + adjusted_groups * adjusted_groups * adjusted_groups
    };

    // Output the result
    println!("{}", result);
    io::stdout().flush().expect("Failed to flush stdout");
}
